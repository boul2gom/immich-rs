use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::visit_mut::VisitMut;

use super::openapi::{BodyKind, EndpointDef, ParamDef, ParamLocation};
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
    let escaped = crate::common::naming::escape_rust_keyword(&snaked);
    format_ident!("{escaped}")
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
    module_path: &str,
    endpoints: &[EndpointDef],
    child_modules: &[String],
    type_map: &std::collections::HashMap<String, String, S>,
) -> TokenStream {
    let last_part = module_path.split('/').next_back().unwrap_or(module_path);
    let mod_ident = format_ident!("{}", snake_to_camel(last_part));

    let mut sorted_endpoints: Vec<&EndpointDef> = endpoints.iter().collect();
    sorted_endpoints.sort_by_key(|e| &e.operation_id);

    let mut structs = Vec::new();
    let mut methods = Vec::new();

    for child in child_modules {
        let child_mod_ident = format_ident!("{child}");
        let child_struct_ident = format_ident!("{}", snake_to_camel(child));
        let child_doc = format!(" Returns a sub-client handle for [`{child}::{child_struct_ident}`] endpoints.");
        methods.push(quote! {
            #[doc = #child_doc]
            #[must_use]
            pub const fn #child_mod_ident(&self) -> #child_mod_ident::#child_struct_ident<'a> {
                #child_mod_ident::#child_struct_ident::new(self.client)
            }
        });
    }

    for e in sorted_endpoints {
        let (s, m) = emit_endpoint(module_path, last_part, e, type_map);
        structs.push(s);
        methods.push(m);
    }

    let module_struct_doc = format!(" Client handle for `{mod_ident}` endpoints.");
    quote! {
        #(#structs)*

        #[doc = #module_struct_doc]
        #[derive(Debug, Clone, Copy)]
        pub struct #mod_ident<'a> {
            client: &'a crate::ImmichClient,
        }

        impl<'a> #mod_ident<'a> {
            #[inline]
            pub(crate) const fn new(client: &'a crate::ImmichClient) -> Self {
                Self { client }
            }

            #(#methods)*
        }
    }
}

fn build_doc_lines(module_path: &str, endpoint: &EndpointDef) -> Vec<String> {
    let mut doc_lines = Vec::new();
    if let Some(summary) = &endpoint.summary {
        let s = crate::common::doc::sanitize_doc_line(summary);
        if !s.is_empty() {
            doc_lines.push(format!(" {s}"));
        }
    }
    if let Some(desc) = &endpoint.description {
        if !doc_lines.is_empty() {
            doc_lines.push(String::new());
        }
        for line in desc.lines() {
            let s = crate::common::doc::sanitize_doc_line(line);
            if s.is_empty() {
                doc_lines.push(String::new());
            } else {
                doc_lines.push(format!(" {s}"));
            }
        }
    }
    if doc_lines.is_empty() {
        doc_lines.push(format!(
            " Calls `{}` `{}`.",
            endpoint.method.to_uppercase(),
            endpoint.path
        ));
    }

    let all_params_len = endpoint.params.len() + usize::from(endpoint.body.is_some());
    if all_params_len > 0 {
        doc_lines.push(String::new());
        doc_lines.push(" ### Parameters".to_string());
        doc_lines.push(String::new());
        doc_lines.push(" | Name | Type | Required | Description |".to_string());
        doc_lines.push(" |------|------|:--------:|-------------|".to_string());
        for p in &endpoint.params {
            let req = if p.required { "✓" } else { "" };
            let param_snake = camel_to_snake(&p.name);
            let p_rust_name = crate::common::naming::escape_rust_keyword(&param_snake);
            let ty_display = if let Some(id_name) =
                crate::common::id_mapping::resolve_endpoint_param_id(&endpoint.operation_id, module_path, &param_snake)
            {
                if p.rust_type == "uuid::Uuid" {
                    id_name.to_string()
                } else if p.rust_type == "Vec<uuid::Uuid>" {
                    format!("Vec<{id_name}>")
                } else {
                    p.rust_type.clone()
                }
            } else {
                p.rust_type.clone()
            };
            let desc = crate::common::doc::sanitize_table_cell(p.description.as_deref().unwrap_or(""));
            doc_lines.push(format!(" | `{p_rust_name}` | `{ty_display}` | {req} | {desc} |"));
        }
        if let Some(body) = &endpoint.body {
            match body.kind {
                BodyKind::Multipart => {
                    doc_lines.push(" | `form` | `reqwest::multipart::Form` | ✓ | Multipart form payload |".to_string());
                }
                BodyKind::FormUrlEncoded => {
                    doc_lines.push(format!(
                        " | `body` | `{}` | ✓ | Form URL-encoded request body |",
                        body.rust_type
                    ));
                }
                BodyKind::Json => {
                    doc_lines.push(format!(" | `body` | `{}` | ✓ | JSON request body |", body.rust_type));
                }
            }
        }
    }

    if !endpoint.responses_docs.is_empty() {
        doc_lines.push(String::new());
        doc_lines.push(" ### Responses".to_string());
        doc_lines.push(String::new());
        doc_lines.push(" | Code | Description |".to_string());
        doc_lines.push(" |------|-------------|".to_string());
        for (code, desc) in &endpoint.responses_docs {
            let clean_desc = crate::common::doc::sanitize_table_cell(desc);
            doc_lines.push(format!(" | `{code}` | {clean_desc} |"));
        }
    }

    if let Some(ret) = &endpoint.response_type {
        doc_lines.push(String::new());
        doc_lines.push(format!(" **Returns:** `{ret}`"));
    }

    doc_lines.push(String::new());
    doc_lines.push(" # Errors".to_string());
    doc_lines.push(" Returns an [`Error`](crate::Error) if request validation fails, the server returns an error status, or deserialization fails.".to_string());

    doc_lines
}

struct ResolvedParamType {
    inner_ty: syn::Type,
    elem_ty: Option<syn::Type>,
    is_into_scalar: bool,
    is_into_elem: bool,
}

fn resolve_param_type<S: std::hash::BuildHasher>(
    endpoint: &EndpointDef,
    module_path: &str,
    param: &ParamDef,
    type_map: &std::collections::HashMap<String, String, S>,
) -> ResolvedParamType {
    let param_snake = camel_to_snake(&param.name);
    let id_opt =
        crate::common::id_mapping::resolve_endpoint_param_id(&endpoint.operation_id, module_path, &param_snake);

    if param.is_array {
        if param.rust_type == "Vec<uuid::Uuid>"
            && let Some(id_name) = id_opt
        {
            let id_ident = format_ident!("{id_name}");
            let elem_ty: syn::Type = syn::parse_quote!(immich_models::id::#id_ident);
            let inner_ty: syn::Type = syn::parse_quote!(Vec<#elem_ty>);
            return ResolvedParamType {
                inner_ty,
                elem_ty: Some(elem_ty),
                is_into_scalar: false,
                is_into_elem: true,
            };
        }

        let inner_ty = map_type(&param.rust_type, type_map);
        let elem_str = param
            .rust_type
            .strip_prefix("Vec<")
            .and_then(|s| s.strip_suffix('>'))
            .unwrap_or("String");
        let elem_ty = map_type(elem_str, type_map);
        let is_into_elem = matches!(elem_str, "String" | "uuid::Uuid");
        return ResolvedParamType {
            inner_ty,
            elem_ty: Some(elem_ty),
            is_into_scalar: false,
            is_into_elem,
        };
    }

    if param.rust_type == "uuid::Uuid"
        && let Some(id_name) = id_opt
    {
        let id_ident = format_ident!("{id_name}");
        let inner_ty: syn::Type = syn::parse_quote!(immich_models::id::#id_ident);
        return ResolvedParamType {
            inner_ty,
            elem_ty: None,
            is_into_scalar: true,
            is_into_elem: false,
        };
    }

    let inner_ty = map_type(&param.rust_type, type_map);
    ResolvedParamType {
        inner_ty,
        elem_ty: None,
        is_into_scalar: param.rust_type == "String",
        is_into_elem: false,
    }
}

pub(crate) fn emit_endpoint<S: std::hash::BuildHasher>(
    module_path: &str,
    last_part: &str,
    endpoint: &EndpointDef,
    type_map: &std::collections::HashMap<String, String, S>,
) -> (TokenStream, TokenStream) {
    let snake_op = camel_to_snake(&endpoint.operation_id);
    let fn_name = format_ident!("{}", crate::common::naming::escape_rust_keyword(&snake_op));
    let doc_lines = build_doc_lines(module_path, endpoint);

    let method_upper = endpoint.method.to_uppercase();
    let reqwest_method = match method_upper.as_str() {
        "POST" => quote! { reqwest::Method::POST },
        "PUT" => quote! { reqwest::Method::PUT },
        "DELETE" => quote! { reqwest::Method::DELETE },
        "PATCH" => quote! { reqwest::Method::PATCH },
        _ => quote! { reqwest::Method::GET },
    };

    let ret_type = if let Some(t) = &endpoint.response_type {
        let ty = map_type(t, type_map);
        quote! { #ty }
    } else {
        quote! { () }
    };

    let trace_module = module_path;
    let module_struct_name = snake_to_camel(last_part);

    let return_stmt = if endpoint.response_type.is_some() {
        if endpoint.is_raw_response {
            quote! {
                let raw_response = self.client.execute_stream(#trace_module, request).await?;
                Ok(raw_response)
            }
        } else {
            quote! {
                let response = self.client.execute::<#ret_type>(#trace_module, request).await?;
                Ok(response)
            }
        }
    } else {
        quote! {
            self.client.execute_unit(#trace_module, request).await?;
            Ok(())
        }
    };

    emit_builder_endpoint(
        endpoint,
        type_map,
        module_path,
        &module_struct_name,
        trace_module,
        &snake_op,
        &fn_name,
        &doc_lines,
        &reqwest_method,
        &method_upper,
        &ret_type,
        &return_stmt,
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn emit_builder_endpoint<S: std::hash::BuildHasher>(
    endpoint: &EndpointDef,
    type_map: &std::collections::HashMap<String, String, S>,
    module_path: &str,
    module_struct_name: &str,
    trace_module: &str,
    snake_op: &str,
    fn_name: &proc_macro2::Ident,
    doc_lines: &[String],
    reqwest_method: &TokenStream,
    method_str: &str,
    ret_type: &TokenStream,
    return_stmt: &TokenStream,
) -> (TokenStream, TokenStream) {
    let camel_op = snake_to_camel(snake_op);
    let builder_ident = format_ident!("{camel_op}Builder");
    let is_multipart = endpoint.body.as_ref().is_some_and(|b| b.kind == BodyKind::Multipart);

    let mut req_fn_args = Vec::new();
    let mut builder_fields = vec![
        quote! { client: &'a crate::ImmichClient },
        quote! { request_config: crate::core::RequestConfig },
    ];
    let mut builder_inits = vec![quote! { request_config: crate::core::RequestConfig::default() }];
    let mut setters = vec![quote! {
        /// Overrides or sets the timeout for this specific request.
        pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
            self.request_config.set_timeout(timeout);
            self
        }

        /// Adds a custom HTTP header to this specific request.
        ///
        /// Invalid header names or values are recorded and surfaced as [`Error::Validation`](crate::Error::Validation)
        /// when the request is sent.
        pub fn header(
            mut self,
            key: impl TryInto<reqwest::header::HeaderName>,
            value: impl TryInto<reqwest::header::HeaderValue>,
        ) -> Self {
            self.request_config.insert_header(key, value);
            self
        }
    }];

    let mut header_params = Vec::new();
    let mut array_query_stmts = Vec::new();
    let mut scalar_query_fields = Vec::new();
    let mut scalar_query_inits = Vec::new();

    for param in &endpoint.params {
        let p_name = param_ident(endpoint, &param.name);
        let resolved = resolve_param_type(endpoint, module_path, param, type_map);
        let inner_ty = &resolved.inner_ty;
        let field_ty: syn::Type = if param.required {
            inner_ty.clone()
        } else {
            syn::parse_quote!(Option<#inner_ty>)
        };

        builder_fields.push(quote! { #p_name: #field_ty });

        if param.required {
            if let Some(elem_ty) = &resolved.elem_ty {
                if resolved.is_into_elem {
                    req_fn_args.push(quote! { #p_name: impl IntoIterator<Item = impl Into<#elem_ty>> });
                    builder_inits.push(quote! { #p_name: #p_name.into_iter().map(Into::into).collect() });
                } else {
                    req_fn_args.push(quote! { #p_name: impl IntoIterator<Item = #elem_ty> });
                    builder_inits.push(quote! { #p_name: #p_name.into_iter().collect() });
                }
            } else if resolved.is_into_scalar {
                req_fn_args.push(quote! { #p_name: impl Into<#inner_ty> });
                builder_inits.push(quote! { #p_name: #p_name.into() });
            } else {
                req_fn_args.push(quote! { #p_name: #inner_ty });
                builder_inits.push(quote! { #p_name });
            }
        } else {
            builder_inits.push(quote! { #p_name: None });
            let raw_setter_doc = param
                .description
                .as_deref()
                .map_or_else(|| format!("Sets `{}`.", param.name), |d| d.replace('\n', " "));
            let setter_doc = format!(" {}", crate::common::doc::sanitize_doc_line(&raw_setter_doc));
            let set_p_name = format_ident!("set_{p_name}");
            let set_doc = format!(" Sets or clears `{}`.", param.name);

            if let Some(elem_ty) = &resolved.elem_ty {
                if resolved.is_into_elem {
                    setters.push(quote! {
                        #[doc = #setter_doc]
                        pub fn #p_name(mut self, value: impl IntoIterator<Item = impl Into<#elem_ty>>) -> Self {
                            self.#p_name = Some(value.into_iter().map(Into::into).collect());
                            self
                        }

                        #[doc = #set_doc]
                        pub fn #set_p_name(mut self, value: Option<#inner_ty>) -> Self {
                            self.#p_name = value;
                            self
                        }
                    });
                } else {
                    setters.push(quote! {
                        #[doc = #setter_doc]
                        pub fn #p_name(mut self, value: impl IntoIterator<Item = #elem_ty>) -> Self {
                            self.#p_name = Some(value.into_iter().collect());
                            self
                        }

                        #[doc = #set_doc]
                        pub fn #set_p_name(mut self, value: Option<#inner_ty>) -> Self {
                            self.#p_name = value;
                            self
                        }
                    });
                }
            } else if resolved.is_into_scalar {
                setters.push(quote! {
                    #[doc = #setter_doc]
                    pub fn #p_name(mut self, value: impl Into<#inner_ty>) -> Self {
                        self.#p_name = Some(value.into());
                        self
                    }

                    #[doc = #set_doc]
                    pub fn #set_p_name(mut self, value: Option<#inner_ty>) -> Self {
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
            ParamLocation::Path => {}
            ParamLocation::Query => {
                let name_str = &param.name;
                if param.is_array {
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
                } else if param.required {
                    scalar_query_fields.push(quote! {
                        #[serde(rename = #name_str)]
                        #p_name: &'b #inner_ty
                    });
                    scalar_query_inits.push(quote! {
                        #p_name: &self.#p_name
                    });
                } else {
                    scalar_query_fields.push(quote! {
                        #[serde(rename = #name_str, skip_serializing_if = "Option::is_none")]
                        #p_name: Option<&'b #inner_ty>
                    });
                    scalar_query_inits.push(quote! {
                        #p_name: self.#p_name.as_ref()
                    });
                }
            }
            ParamLocation::Header => {
                let name_str = &param.name;
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

    if let Some(body) = &endpoint.body {
        match body.kind {
            BodyKind::Multipart => {
                req_fn_args.push(quote! { form: reqwest::multipart::Form });
                builder_fields.push(quote! { form: reqwest::multipart::Form });
                builder_inits.push(quote! { form });
            }
            BodyKind::Json | BodyKind::FormUrlEncoded => {
                let body_ty = map_type(&body.rust_type, type_map);
                req_fn_args.push(quote! { body: impl Into<#body_ty> });
                builder_fields.push(quote! { body: #body_ty });
                builder_inits.push(quote! { body: body.into() });
            }
        }
    }

    let mut local_struct_defs = quote! {};
    let mut query_params = Vec::new();
    if !scalar_query_fields.is_empty() {
        local_struct_defs = quote! {
            #[derive(serde::Serialize)]
            struct QueryParams<'b> {
                #(#scalar_query_fields),*
            }
        };
        query_params.push(quote! {
            let query = QueryParams {
                #(#scalar_query_inits),*
            };
            request = request.query(&query);
        });
    }
    query_params.extend(array_query_stmts);

    let endpoint_path = &endpoint.path;
    let has_path_params = endpoint_path.contains('{');
    let prepare_stmt = if has_path_params {
        let mut bindings = Vec::new();
        let mut segment_exprs = Vec::new();
        for (idx, seg) in endpoint_path.trim_start_matches('/').split('/').enumerate() {
            if let Some(raw_param) = seg.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                if let Some(p) = endpoint
                    .params
                    .iter()
                    .find(|p| p.in_ == ParamLocation::Path && p.name == raw_param)
                {
                    let p_name = param_ident(endpoint, &p.name);
                    if p.rust_type == "String" {
                        segment_exprs.push(quote! { crate::core::PathSegment::Param(self.#p_name.as_str()) });
                    } else {
                        let seg_var = format_ident!("__seg_{idx}");
                        bindings.push(quote! { let #seg_var = self.#p_name.to_string(); });
                        segment_exprs.push(quote! { crate::core::PathSegment::Param(#seg_var.as_str()) });
                    }
                }
            } else {
                segment_exprs.push(quote! { crate::core::PathSegment::Static(#seg) });
            }
        }
        quote! {
            #(#bindings)*
            let mut request = self.client.prepare_request_segments(#reqwest_method, &[#(#segment_exprs),*])?;
        }
    } else {
        quote! {
            let mut request = self.client.prepare_request(#reqwest_method, #endpoint_path)?;
        }
    };

    let mut request_builder = quote! {
        #prepare_stmt
        request = self.request_config.apply(request)?;
        #(#header_params)*
        #(#query_params)*
    };

    if let Some(body) = &endpoint.body {
        match body.kind {
            BodyKind::Multipart => {
                request_builder = quote! {
                    #request_builder
                    request = request.multipart(self.form);
                };
            }
            BodyKind::FormUrlEncoded => {
                request_builder = quote! {
                    #request_builder
                    let encoded_body = serde_urlencoded::to_string(&self.body)
                        .map_err(|e| crate::Error::Validation(format!("Failed to URL-encode form body: {e}")))?;
                    request = request
                        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                        .body(encoded_body);
                };
            }
            BodyKind::Json => {
                request_builder = quote! {
                    #request_builder
                    request = request.json(&self.body);
                };
            }
        }
    }

    let derive_tokens = if is_multipart {
        quote! { #[derive(Debug)] }
    } else {
        quote! { #[derive(Debug, Clone)] }
    };

    let (into_paginator_tokens, paginator_struct_tokens) = if endpoint.operation_id == "searchAssets" {
        crate::client::paginator::generate_search_body_paginator(&builder_ident, "search_assets")
    } else if endpoint.operation_id == "searchSmart" {
        crate::client::paginator::generate_search_body_paginator(&builder_ident, "search_smart")
    } else {
        crate::client::paginator::generate_paginator(&builder_ident, endpoint, type_map)
    };

    let builder_doc = format!(" Fluent request builder for [`{module_struct_name}::{fn_name}`].");
    let builder_impl_header = if into_paginator_tokens.is_empty() {
        quote! { impl #builder_ident<'_> }
    } else {
        quote! { impl<'a> #builder_ident<'a> }
    };

    let struct_tokens = quote! {
        #[doc = #builder_doc]
        #derive_tokens
        #[must_use = "request builders are lazy and do nothing unless awaited or sent"]
        pub struct #builder_ident<'a> {
            #(#builder_fields),*
        }

        #builder_impl_header {
            #(#setters)*

            #into_paginator_tokens

            /// Sends the HTTP request and awaits the server response.
            ///
            /// # Errors
            ///
            /// Returns an [`Error`](crate::Error) if request validation fails, the API returns an HTTP 4xx/5xx status, or deserialization fails.
            pub async fn send(self) -> Result<#ret_type, crate::Error> {
                #local_struct_defs
                tracing::trace!(
                    module = #trace_module,
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
