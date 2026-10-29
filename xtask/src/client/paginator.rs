use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::client::detect_pagination::PaginationKind;
use crate::client::openapi::EndpointDef;

fn resolve_inner_type<S: std::hash::BuildHasher>(
    endpoint: &EndpointDef,
    type_map: &std::collections::HashMap<String, String, S>,
) -> Option<syn::Type> {
    let pag_def = endpoint.paginator.as_ref()?;
    let mut inner_type_fqdn = pag_def.item_inner_type.clone();
    if let Some(mapped) = type_map.get(&inner_type_fqdn) {
        inner_type_fqdn = mapped.clone();
    } else if let Some(resp_type) = &endpoint.response_type {
        let resp_mapped = type_map.get(resp_type).cloned().unwrap_or_else(|| resp_type.clone());
        if let Some(last_colons) = resp_mapped.rfind("::") {
            let mod_path = &resp_mapped[..last_colons];
            inner_type_fqdn = format!("{mod_path}::{inner_type_fqdn}");
        }
    }
    Some(syn::parse_str(&inner_type_fqdn).unwrap_or_else(|_| syn::parse_quote!(serde_json::Value)))
}

/// Generates AWS-SDK style paginator:
/// 1. `.into_paginator(self) -> #paginator_ident<'a>` on `#builder_ident<'a>`
/// 2. Struct `#paginator_ident<'a>` with `.pages(self)` and `.items(self)`.
#[must_use]
pub fn generate_paginator<S: std::hash::BuildHasher>(
    builder_ident: &proc_macro2::Ident,
    endpoint: &EndpointDef,
    type_map: &std::collections::HashMap<String, String, S>,
) -> (TokenStream, TokenStream) {
    let Some(pag_def) = endpoint.paginator.as_ref() else {
        return (quote! {}, quote! {});
    };
    let Some(inner_type_tokens) = resolve_inner_type(endpoint, type_map) else {
        return (quote! {}, quote! {});
    };

    let builder_str = builder_ident.to_string();
    let base_name = builder_str.strip_suffix("Builder").unwrap_or(&builder_str);
    let paginator_ident = format_ident!("{base_name}Paginator");

    let into_paginator_method = quote! {
        /// Creates a paginator for this request, providing streams over pages or individual items.
        #[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
        pub fn into_paginator(self) -> #paginator_ident<'a> {
            #paginator_ident { builder: self }
        }
    };

    let (pages_stream_impl, extra_methods) = match &pag_def.kind {
        PaginationKind::Wrapped {
            has_next_field,
            items_field,
            is_cursor: true,
        } => {
            let items_ident = format_ident!("{items_field}");
            let has_next_ident = format_ident!("{has_next_field}");

            let pages_body = quote! {
                let mut builder = self.builder;
                async_stream::try_stream! {
                    loop {
                        let res = builder.clone().send().await?;
                        let items = res.#items_ident;
                        let is_empty = items.is_empty();
                        let next = res.#has_next_ident;
                        yield items;
                        if is_empty {
                            break;
                        }
                        match next {
                            Some(c) if !c.is_empty() => builder = builder.cursor(c),
                            _ => break,
                        }
                    }
                }
            };
            (pages_body, quote! {})
        }
        PaginationKind::Wrapped {
            has_next_field,
            items_field,
            is_cursor: false,
        } => {
            let items_ident = format_ident!("{items_field}");
            let has_next_ident = format_ident!("{has_next_field}");

            let pages_body = quote! {
                let builder = self.builder;
                let mut current_page = builder.page.unwrap_or(1);
                async_stream::try_stream! {
                    use crate::utils::IsTrue;
                    loop {
                        let res = builder.clone().page(current_page).send().await?;
                        let items = res.#items_ident;
                        let is_empty = items.is_empty();
                        let has_next = res.#has_next_ident;
                        yield items;
                        if is_empty || !has_next.is_true() {
                            break;
                        }
                        current_page += 1;
                    }
                }
            };
            (pages_body, quote! {})
        }
        PaginationKind::DirectPage {
            page_param: _,
            size_param,
        } => {
            let size_check = if let Some(sp) = size_param {
                let sp_ident = format_ident!("{sp}");
                quote! {
                    else if let Some(expected) = builder.#sp_ident {
                        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
                        if count_i64 < expected {
                            break;
                        }
                    }
                }
            } else {
                quote! {}
            };

            let extra = if let Some(sp) = size_param {
                let sp_ident = format_ident!("{sp}");
                quote! {
                    /// Sets the page size for subsequent page requests.
                    pub fn page_size(mut self, size: i64) -> Self {
                        self.builder = self.builder.#sp_ident(size);
                        self
                    }
                }
            } else {
                quote! {}
            };

            let pages_body = quote! {
                let builder = self.builder;
                let mut current_page = builder.page.unwrap_or(1);
                async_stream::try_stream! {
                    loop {
                        let items = builder.clone().page(current_page).send().await?;
                        let count = items.len();
                        yield items;
                        if count == 0 {
                            break;
                        } #size_check
                        current_page += 1;
                    }
                }
            };
            (pages_body, extra)
        }
        PaginationKind::DirectOffset { take_param, skip_param } => {
            let take_ident = format_ident!("{take_param}");
            let skip_ident = format_ident!("{skip_param}");

            let pages_body = quote! {
                let mut builder = self.builder;
                let mut current_skip = builder.#skip_ident.unwrap_or(0);
                let take = builder.#take_ident.unwrap_or(100);
                async_stream::try_stream! {
                    loop {
                        let items = builder.clone().#skip_ident(current_skip).#take_ident(take).send().await?;
                        let count = items.len();
                        yield items;
                        let count_i64 = i64::try_from(count).unwrap_or(i64::MAX);
                        if count_i64 < take {
                            break;
                        }
                        current_skip += count_i64;
                    }
                }
            };
            (pages_body, quote! {})
        }
    };

    let paginator_doc = format!(" Paginator for [`{builder_ident}`].");

    let paginator_struct = quote! {
        #[doc = #paginator_doc]
        #[derive(Debug, Clone)]
        #[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
        pub struct #paginator_ident<'a> {
            builder: #builder_ident<'a>,
        }

        impl<'a> #paginator_ident<'a> {
            #extra_methods

            /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding pages of items.
            #[must_use = "streams do nothing unless polled"]
            pub fn pages(self) -> impl futures_core::Stream<Item = Result<Vec<#inner_type_tokens>, crate::Error>> + 'a {
                #pages_stream_impl
            }

            /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding individual items, flattened across pages.
            #[must_use = "streams do nothing unless polled"]
            pub fn items(self) -> impl futures_core::Stream<Item = Result<#inner_type_tokens, crate::Error>> + 'a {
                use futures_util::StreamExt;
                self.pages().flat_map(|page_res| match page_res {
                    Ok(items) => futures_util::stream::iter(items.into_iter().map(Ok)).left_stream(),
                    Err(err) => futures_util::stream::once(async move { Err(err) }).right_stream(),
                })
            }
        }
    };

    (into_paginator_method, paginator_struct)
}

/// Specialized paginators for Search endpoints (`search_assets`, `search_smart`).
#[must_use]
pub fn generate_search_body_paginator(
    builder_ident: &proc_macro2::Ident,
    method_name: &str,
) -> (TokenStream, TokenStream) {
    let builder_str = builder_ident.to_string();
    let base_name = builder_str.strip_suffix("Builder").unwrap_or(&builder_str);
    let paginator_ident = format_ident!("{base_name}Paginator");
    let method_ident = format_ident!("{method_name}");

    let into_paginator_method = quote! {
        /// Creates a paginator for this request, providing streams over pages or individual items.
        #[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
        pub fn into_paginator(self) -> #paginator_ident<'a> {
            #paginator_ident { builder: self }
        }
    };

    let paginator_doc = format!(" Paginator for [`{builder_ident}`].");

    let paginator_struct = quote! {
        #[doc = #paginator_doc]
        #[derive(Debug, Clone)]
        #[must_use = "paginators are lazy and do nothing unless .pages() or .items() is called"]
        pub struct #paginator_ident<'a> {
            builder: #builder_ident<'a>,
        }

        impl<'a> #paginator_ident<'a> {
            /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding pages of items.
            #[must_use = "streams do nothing unless polled"]
            pub fn pages(self) -> impl futures_core::Stream<Item = Result<Vec<immich_models::assets::AssetResponseDTO>, crate::Error>> + 'a {
                let client = self.builder.client;
                let mut body = (*self.builder.body).clone();
                async_stream::try_stream! {
                    loop {
                        let res = client.search().#method_ident(&body).send().await?;
                        let assets = res.assets;
                        let is_empty = assets.items.is_empty();
                        let next = assets
                            .next_page
                            .as_deref()
                            .and_then(|p| p.parse::<u64>().ok())
                            .and_then(std::num::NonZeroU64::new);
                        yield assets.items;
                        match next {
                            Some(page) if !is_empty => body.page = Some(page),
                            _ => break,
                        }
                    }
                }
            }

            /// Returns an asynchronous [`Stream`](futures_core::Stream) yielding individual items, flattened across pages.
            #[must_use = "streams do nothing unless polled"]
            pub fn items(self) -> impl futures_core::Stream<Item = Result<immich_models::assets::AssetResponseDTO, crate::Error>> + 'a {
                use futures_util::StreamExt;
                self.pages().flat_map(|page_res| match page_res {
                    Ok(items) => futures_util::stream::iter(items.into_iter().map(Ok)).left_stream(),
                    Err(err) => futures_util::stream::once(async move { Err(err) }).right_stream(),
                })
            }
        }
    };

    (into_paginator_method, paginator_struct)
}
