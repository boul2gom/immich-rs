use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::visit_mut::VisitMut;

use super::openapi::{EndpointDef, ParamLocation};
use crate::common::{camel_to_snake, snake_to_camel};

struct TypeReplacer<'a, S: std::hash::BuildHasher> {
    type_map: &'a std::collections::HashMap<String, String, S>,
}

impl<S: std::hash::BuildHasher> VisitMut for TypeReplacer<'_, S> {
    fn visit_type_path_mut(&mut self, i: &mut syn::TypePath) {
        if i.qself.is_none() && i.path.segments.len() == 1 {
            let ident = &i.path.segments[0].ident;
            let ident_str = ident.to_string();
            if let Some(fqdn) = self.type_map.get(&ident_str)
                && let Ok(new_path) = syn::parse_str::<syn::Path>(fqdn)
            {
                i.path = new_path;
                return;
            }
        }
        syn::visit_mut::visit_type_path_mut(self, i);
    }
}

pub(crate) fn map_type<S: std::hash::BuildHasher>(
    ty_str: &str,
    type_map: &std::collections::HashMap<String, String, S>,
) -> syn::Type {
    let mut rust_type = syn::parse_str::<syn::Type>(ty_str).unwrap_or_else(|_| syn::parse_quote!(String));
    let mut replacer = TypeReplacer { type_map };
    replacer.visit_type_mut(&mut rust_type);
    rust_type
}

pub(crate) fn param_ident(_endpoint: &EndpointDef, param_name: &str) -> proc_macro2::Ident {
    let snaked = camel_to_snake(param_name);
    match snaked.as_str() {
        // Rust keywords are suffixed with an underscore, matching `immich-models` field names.
        "type" | "ref" | "for" | "where" | "match" | "in" => format_ident!("{snaked}_"),
        _ => format_ident!("{snaked}"),
    }
}

pub(crate) fn deprecated_attr(endpoint: &EndpointDef) -> TokenStream {
    if endpoint.deprecated {
        quote! { #[deprecated(note = "Deprecated in the Immich OpenAPI specification.")] }
    } else {
        quote! {}
    }
}

#[must_use]
pub fn emit_client_module<S: std::hash::BuildHasher>(
    module_name: &str,
    endpoints: &[EndpointDef],
    type_map: &std::collections::HashMap<String, String, S>,
) -> TokenStream {
    let mod_ident = format_ident!("{}", snake_to_camel(module_name));

    let mut sorted_endpoints: Vec<&EndpointDef> = endpoints.iter().collect();
    sorted_endpoints.sort_by_key(|e| &e.operation_id);

    let mut structs = Vec::new();
    let mut methods = Vec::new();

    for e in sorted_endpoints {
        let (s, m) = emit_endpoint(module_name, e, type_map);
        structs.push(s);
        methods.push(m);
    }

    let module_struct_doc = format!("Client handle for `{mod_ident}` endpoints.");
    quote! {
        #(#structs)*

        #[doc = #module_struct_doc]
        #[derive(Debug, Clone, Copy)]
        pub struct #mod_ident<'a> {
            client: &'a crate::ImmichClient,
        }

        impl<'a> #mod_ident<'a> {
            pub(crate) fn new(client: &'a crate::ImmichClient) -> Self {
                Self { client }
            }

            #(#methods)*
        }
    }
}

fn build_doc_lines(endpoint: &EndpointDef) -> Vec<String> {
    let mut doc_lines = Vec::new();
    if let Some(summary) = &endpoint.summary {
        doc_lines.push(format!(" {summary}"));
    }
    if let Some(desc) = &endpoint.description {
        for line in desc.split('\n') {
            doc_lines.push(format!(" {line}"));
        }
    }

    let all_params_len = endpoint.params.len() + usize::from(endpoint.body.is_some());
    if all_params_len > 0 {
        doc_lines.push(String::new());
        doc_lines.push(" ### Parameters".to_string());
        doc_lines.push(String::new());
        doc_lines.push(" | Name | Type | Required | Description |".to_string());
        doc_lines.push(" |--------|--------|:--------:|--------|".to_string());
        for p in &endpoint.params {
            let req = if p.required { "✓" } else { "" };
            let desc = p.description.clone().unwrap_or_default().replace('\n', " ");
            doc_lines.push(format!(" | `{}` | `{}` | {} | {} |", p.name, p.rust_type, req, desc));
        }
        if let Some(body) = &endpoint.body {
            doc_lines.push(format!(" | `body` | `{}` | ✅ | Request body |", body.rust_type));
        }
        doc_lines.push(String::new());
    }

    if !endpoint.responses_docs.is_empty() {
        doc_lines.push(" ### Responses".to_string());
        doc_lines.push(String::new());
        doc_lines.push(" | Code | Description |".to_string());
        doc_lines.push(" |--------|--------|".to_string());
        for (code, desc) in &endpoint.responses_docs {
            doc_lines.push(format!(" | `{}` | {} |", code, desc.replace('\n', " ")));
        }
        doc_lines.push(String::new());
    }

    if let Some(ret) = &endpoint.response_type {
        doc_lines.push(format!(" **Returns:** `{ret}`"));
        doc_lines.push(String::new());
    }

    doc_lines
}

fn extract_url_ordered_params(path: &str) -> Vec<String> {
    let mut url_ordered_params = Vec::new();
    let mut path_chars = path.chars().peekable();
    while let Some(c) = path_chars.next() {
        if c == '{' {
            let mut param = String::new();
            for c in path_chars.by_ref() {
                if c == '}' {
                    break;
                }
                param.push(c);
            }
            url_ordered_params.push(crate::common::camel_to_snake(&param));
        }
    }
    url_ordered_params
}

pub(crate) fn resolve_param_id_type(module_name: &str, param_name: &str, rust_type: &str) -> Option<syn::Type> {
    if rust_type != "uuid::Uuid" {
        return None;
    }
    let snaked = camel_to_snake(param_name);
    let id_type = match snaked.as_str() {
        "album_id" => "AlbumId",
        "asset_id" | "primary_asset_id" => "AssetId",
        "user_id" => "UserId",
        "tag_id" => "TagId",
        "library_id" => "LibraryId",
        "api_key_id" | "key_id" => "ApiKeyId",
        "person_id" => "PersonId",
        "memory_id" => "MemoryId",
        "stack_id" => "StackId",
        "activity_id" => "ActivityId",
        "partner_id" => "PartnerId",
        "workflow_id" => "WorkflowId",
        "id" => match module_name {
            "albums" => "AlbumId",
            "assets" | "asset_files" => "AssetId",
            "users" | "users_admin" => "UserId",
            "tags" => "TagId",
            "libraries" => "LibraryId",
            "api_keys" => "ApiKeyId",
            "people" | "person" => "PersonId",
            "memories" => "MemoryId",
            "stacks" => "StackId",
            "activities" => "ActivityId",
            "partners" => "PartnerId",
            "workflows" => "WorkflowId",
            _ => return None,
        },
        _ => return None,
    };
    let ident = format_ident!("{id_type}");
    Some(syn::parse_quote!(immich_models::id::#ident))
}

pub(crate) fn emit_endpoint<S: std::hash::BuildHasher>(
    module_name: &str,
    endpoint: &EndpointDef,
    type_map: &std::collections::HashMap<String, String, S>,
) -> (TokenStream, TokenStream) {
    let snake_op = camel_to_snake(&endpoint.operation_id);
    let fn_name = format_ident!("{snake_op}");
    let doc_lines = build_doc_lines(endpoint);

    let method_upper = endpoint.method.to_uppercase();
    let reqwest_method = match method_upper.as_str() {
        "POST" => quote! { reqwest::Method::POST },
        "PUT" => quote! { reqwest::Method::PUT },
        "DELETE" => quote! { reqwest::Method::DELETE },
        "PATCH" => quote! { reqwest::Method::PATCH },
        _ => quote! { reqwest::Method::GET },
    };

    let has_body = endpoint.body.is_some();
    let is_multipart = endpoint.body.as_ref().is_some_and(|b| b.is_multipart);

    let ret_type = if let Some(t) = &endpoint.response_type {
        let ty = map_type(t, type_map);
        quote! { #ty }
    } else {
        quote! { () }
    };

    let return_stmt = if endpoint.response_type.is_some() {
        if endpoint.is_raw_response {
            quote! {
                let raw_response = self.client.execute_stream(#module_name, request).await?;
                Ok(raw_response)
            }
        } else {
            quote! {
                let response = self.client.execute::<#ret_type>(#module_name, request).await?;
                Ok(response)
            }
        }
    } else {
        quote! {
            self.client.execute_unit(#module_name, request).await?;
            Ok(())
        }
    };

    let endpoint_path = endpoint.path.clone();
    let module_name_camel = crate::common::snake_to_camel(module_name);
    let url_ordered_params = extract_url_ordered_params(&endpoint.path);

    emit_builder_endpoint(
        endpoint,
        type_map,
        module_name,
        &snake_op,
        &fn_name,
        &doc_lines,
        &reqwest_method,
        &method_upper,
        &endpoint_path,
        &module_name_camel,
        &url_ordered_params,
        has_body,
        is_multipart,
        &ret_type,
        &return_stmt,
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn emit_builder_endpoint<S: std::hash::BuildHasher>(
    endpoint: &EndpointDef,
    type_map: &std::collections::HashMap<String, String, S>,
    module_name: &str,
    snake_op: &str,
    fn_name: &proc_macro2::Ident,
    doc_lines: &[String],
    reqwest_method: &TokenStream,
    method_str: &str,
    endpoint_path: &str,
    module_name_camel: &str,
    url_ordered_params: &[String],
    has_body: bool,
    is_multipart: bool,
    ret_type: &TokenStream,
    return_stmt: &TokenStream,
) -> (TokenStream, TokenStream) {
    let camel_op = snake_to_camel(snake_op);
    let builder_ident = format_ident!("{camel_op}Builder");
    let args_alias_ident = format_ident!("{camel_op}Args");

    let mut path_str = endpoint.path.clone();
    let mut req_fn_args = Vec::new();
    let mut builder_fields = Vec::new();
    let mut builder_inits = Vec::new();
    let mut setters = Vec::new();
    let mut header_params = Vec::new();
    let mut array_query_stmts = Vec::new();
    let mut multipart_query_fields = Vec::new();
    let mut multipart_query_inits = Vec::new();
    let mut has_scalar_query_param = false;

    if is_multipart {
        builder_fields.push(quote! {
            client: &'a crate::ImmichClient
        });
        builder_fields.push(quote! {
            pub request_config: crate::core::RequestConfig
        });
    } else {
        builder_fields.push(quote! {
            #[serde(skip)]
            client: &'a crate::ImmichClient
        });
        builder_fields.push(quote! {
            #[serde(skip)]
            pub request_config: crate::core::RequestConfig
        });
    }

    builder_inits.push(quote! {
        request_config: crate::core::RequestConfig::default()
    });

    setters.push(quote! {
        /// Overrides or sets the timeout for this specific request.
        pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
            self.request_config.set_timeout(timeout);
            self
        }

        /// Adds a custom HTTP header to this specific request.
        pub fn header(mut self, key: impl TryInto<reqwest::header::HeaderName>, value: impl TryInto<reqwest::header::HeaderValue>) -> Self {
            self.request_config.insert_header(key, value);
            self
        }
    });

    for param in &endpoint.params {
        let p_name = param_ident(endpoint, &param.name);
        let id_type_opt = resolve_param_id_type(module_name, &param.name, &param.rust_type);
        let inner_ty = if let Some(id_ty) = &id_type_opt {
            id_ty.clone()
        } else {
            map_type(&param.rust_type, type_map)
        };
        let field_ty: syn::Type = if param.required {
            inner_ty.clone()
        } else {
            syn::parse_quote!(Option<#inner_ty>)
        };

        if param.required {
            if let Some(id_ty) = &id_type_opt {
                req_fn_args.push(quote! { #p_name: impl Into<#id_ty> });
                builder_inits.push(quote! { #p_name: #p_name.into() });
            } else if param.rust_type == "String" {
                req_fn_args.push(quote! { #p_name: impl Into<String> });
                builder_inits.push(quote! { #p_name: #p_name.into() });
            } else {
                req_fn_args.push(quote! { #p_name: #inner_ty });
                builder_inits.push(quote! { #p_name });
            }
        } else {
            builder_inits.push(quote! { #p_name: None });
            let setter_doc = param.description.as_deref().map_or_else(
                || format!(" Sets `{}`.", param.name),
                |d| format!(" {}", d.replace('\n', " ")),
            );
            let set_p_name = format_ident!("set_{p_name}");
            let set_doc = format!(" Sets or clears `{}`.", param.name);
            if let Some(id_ty) = &id_type_opt {
                setters.push(quote! {
                    #[doc = #setter_doc]
                    pub fn #p_name(mut self, value: impl Into<#id_ty>) -> Self {
                        self.#p_name = Some(value.into());
                        self
                    }

                    #[doc = #set_doc]
                    pub fn #set_p_name(mut self, value: Option<#id_ty>) -> Self {
                        self.#p_name = value;
                        self
                    }
                });
            } else if param.rust_type == "String" {
                setters.push(quote! {
                    #[doc = #setter_doc]
                    pub fn #p_name(mut self, value: impl Into<String>) -> Self {
                        self.#p_name = Some(value.into());
                        self
                    }

                    #[doc = #set_doc]
                    pub fn #set_p_name(mut self, value: Option<String>) -> Self {
                        self.#p_name = value;
                        self
                    }
                });
            } else {
                setters.push(quote! {
                    #[doc = #setter_doc]
                    pub fn #p_name(mut self, value: #inner_ty) -> Self {
                        self.#p_name = Some(value);
                        self
                    }

                    #[doc = #set_doc]
                    pub fn #set_p_name(mut self, value: Option<#inner_ty>) -> Self {
                        self.#p_name = value;
                        self
                    }
                });
            }
        }

        match param.in_ {
            ParamLocation::Path => {
                path_str = path_str.replace(&format!("{{{}}}", param.name), "{}");
                if is_multipart {
                    builder_fields.push(quote! {
                        pub #p_name: #field_ty
                    });
                } else {
                    builder_fields.push(quote! {
                        #[serde(skip)]
                        pub #p_name: #field_ty
                    });
                }
            }
            ParamLocation::Query => {
                let name_str = &param.name;
                if param.is_array {
                    if is_multipart {
                        builder_fields.push(quote! {
                            pub #p_name: #field_ty
                        });
                    } else {
                        builder_fields.push(quote! {
                            #[serde(skip)]
                            pub #p_name: #field_ty
                        });
                    }
                    if param.required {
                        array_query_stmts.push(quote! {
                            for item in &self.#p_name {
                                request = request.query(&[(#name_str, item)]);
                            }
                        });
                    } else {
                        array_query_stmts.push(quote! {
                            if let Some(items) = &self.#p_name {
                                for item in items {
                                    request = request.query(&[(#name_str, item)]);
                                }
                            }
                        });
                    }
                } else {
                    has_scalar_query_param = true;
                    if is_multipart {
                        builder_fields.push(quote! {
                            pub #p_name: #field_ty
                        });
                        if param.required {
                            multipart_query_fields.push(quote! {
                                #[serde(rename = #name_str)]
                                #p_name: &'b #inner_ty
                            });
                            multipart_query_inits.push(quote! {
                                #p_name: &self.#p_name
                            });
                        } else {
                            multipart_query_fields.push(quote! {
                                #[serde(rename = #name_str, skip_serializing_if = "Option::is_none")]
                                #p_name: Option<&'b #inner_ty>
                            });
                            multipart_query_inits.push(quote! {
                                #p_name: self.#p_name.as_ref()
                            });
                        }
                    } else if param.required {
                        builder_fields.push(quote! {
                            #[serde(rename = #name_str)]
                            pub #p_name: #field_ty
                        });
                    } else {
                        builder_fields.push(quote! {
                            #[serde(rename = #name_str, skip_serializing_if = "Option::is_none")]
                            pub #p_name: #field_ty
                        });
                    }
                }
            }
            ParamLocation::Header => {
                let name_str = &param.name;
                if is_multipart {
                    builder_fields.push(quote! {
                        pub #p_name: #field_ty
                    });
                } else {
                    builder_fields.push(quote! {
                        #[serde(skip)]
                        pub #p_name: #field_ty
                    });
                }
                if param.required {
                    if param.rust_type == "String" {
                        header_params.push(quote! {
                            request = request.header(#name_str, self.#p_name.as_str());
                        });
                    } else {
                        header_params.push(quote! {
                            request = request.header(#name_str, self.#p_name.to_string());
                        });
                    }
                } else if param.rust_type == "String" {
                    header_params.push(quote! {
                        if let Some(v) = &self.#p_name {
                            request = request.header(#name_str, v.as_str());
                        }
                    });
                } else {
                    header_params.push(quote! {
                        if let Some(v) = &self.#p_name {
                            request = request.header(#name_str, v.to_string());
                        }
                    });
                }
            }
        }
    }

    if has_body {
        if is_multipart {
            req_fn_args.push(quote! { form: reqwest::multipart::Form });
            builder_fields.push(quote! {
                pub form: reqwest::multipart::Form
            });
            builder_inits.push(quote! { form });
        } else {
            let body = endpoint.body.as_ref().unwrap();
            let body_ty = map_type(&body.rust_type, type_map);
            req_fn_args.push(quote! { body: &'a #body_ty });
            builder_fields.push(quote! {
                #[serde(skip)]
                pub body: &'a #body_ty
            });
            builder_inits.push(quote! { body });
        }
    }

    let mut local_struct_defs = quote! {};
    let mut query_params = Vec::new();
    if has_scalar_query_param {
        if is_multipart {
            local_struct_defs = quote! {
                #[derive(serde::Serialize)]
                struct QueryParams<'b> {
                    #(#multipart_query_fields),*
                }
            };
            query_params.push(quote! {
                let query = QueryParams {
                    #(#multipart_query_inits),*
                };
                request = request.query(&query);
            });
        } else {
            query_params.push(quote! {
                request = request.query(&self);
            });
        }
    }
    query_params.extend(array_query_stmts);

    let mut path_args = Vec::new();
    for param_name_snake in url_ordered_params {
        if let Some(p) = endpoint
            .params
            .iter()
            .find(|p| crate::common::camel_to_snake(&p.name) == *param_name_snake)
        {
            let p_name = param_ident(endpoint, &p.name);
            if p.rust_type == "String" {
                path_args.push(quote! {
                    ::urlencoding::encode(&self.#p_name)
                });
            } else {
                path_args.push(quote! {
                    self.#p_name
                });
            }
        }
    }

    let path_expr = if path_args.is_empty() {
        quote! { #path_str }
    } else {
        quote! { &format!(#path_str, #(#path_args),*) }
    };

    let mut request_builder = quote! {
        let mut request = self.client.prepare_request(#reqwest_method, #path_expr)?;
        request = self.request_config.apply(request);
        #(#header_params)*
        #(#query_params)*
    };

    if has_body {
        if is_multipart {
            request_builder = quote! {
                #request_builder
                request = request.multipart(self.form);
            };
        } else {
            request_builder = quote! {
                #request_builder
                request = request.json(self.body);
            };
        }
    }

    let derive_tokens = if is_multipart {
        quote! { #[derive(Debug)] }
    } else {
        quote! { #[derive(serde::Serialize, Debug, Clone)] }
    };

    let (into_paginator_tokens, paginator_struct_tokens) = if endpoint.operation_id == "searchAssets" {
        crate::client::paginator::generate_search_body_paginator(&builder_ident, "search_assets")
    } else if endpoint.operation_id == "searchSmart" {
        crate::client::paginator::generate_search_body_paginator(&builder_ident, "search_smart")
    } else {
        crate::client::paginator::generate_paginator(&builder_ident, endpoint, type_map)
    };

    let builder_doc = format!(" Fluent request builder for [`{module_name_camel}::{fn_name}`].");
    let alias_doc = format!(" Type alias for [`{builder_ident}`].");

    let struct_tokens = quote! {
        #[doc = #builder_doc]
        #derive_tokens
        #[must_use = "request builders are lazy and do nothing unless awaited or sent"]
        pub struct #builder_ident<'a> {
            #(#builder_fields),*
        }

        #[doc = #alias_doc]
        pub type #args_alias_ident<'a> = #builder_ident<'a>;

        impl<'a> #builder_ident<'a> {
            #(#setters)*

            #into_paginator_tokens

            /// Sends the HTTP request and awaits the server response.
            ///
            /// # Errors
            ///
            /// Returns an error if the API returns an HTTP 4xx/5xx or if deserialization fails.
            pub async fn send(self) -> Result<#ret_type, crate::Error> {
                #local_struct_defs
                tracing::trace!(
                    module = #module_name_camel,
                    method = #method_str,
                    path = #endpoint_path,
                    "HTTP request"
                );
                #request_builder
                #return_stmt
            }
        }

        impl<'a> std::future::IntoFuture for #builder_ident<'a> {
            type Output = Result<#ret_type, crate::Error>;
            type IntoFuture = crate::BoxFuture<'a, Self::Output>;

            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send())
            }
        }

        #paginator_struct_tokens
    };

    let deprecated_attr = deprecated_attr(endpoint);
    let method_tokens = quote! {
        #(#[doc = #doc_lines])*
        #deprecated_attr
        pub fn #fn_name(&self, #(#req_fn_args),*) -> #builder_ident<'a> {
            #builder_ident {
                client: self.client,
                #(#builder_inits),*
            }
        }
    };

    (struct_tokens, method_tokens)
}
