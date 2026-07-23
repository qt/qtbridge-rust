// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::Ident;
use crate::qt_gen_impl;
use qt_gen_impl::qobject_macro_params::QObjectMacroParams;

pub struct QmlElementCode {
    pub register_fn: Option<syn::ItemFn>,
    pub register_impl: syn::ItemImpl,
}

impl ToTokens for QmlElementCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self{ register_fn, register_impl } = self;
        quote! {
            #register_fn
            #register_impl
        }.to_tokens(tokens);
    }
}


pub fn generate_qml_register(struct_ident: &Ident, params: &QObjectMacroParams) -> syn::Result<Option<QmlElementCode>> {

    if params.no_qml_element {
        return Ok(None)
    }

    let qml_register_fn_indent = format_ident!("qml_register_{struct_ident}");
    let register_fn = match params.link_me {
        true => {
            let code = quote! {
                // TODO: make auto registration via 'linkme' dependency an optional cargo feature?
                #[linkme::distributed_slice(qtbridge::qtbridge_runtime::QML_REGISTER_CALLBACKS)]
                #[allow(non_camel_case_types)]
                fn #qml_register_fn_indent() {
                    <#struct_ident as qtbridge::qtbridge_runtime::QmlRegister>::register();
                }
            };
            Some(syn::parse2(code)?)
        },
        false => None
    };

    let struct_name = struct_ident.to_string();
    let is_singleton = params.singleton;

    let uri = std::env::var("CARGO_PKG_NAME")
        .map_err(|err| syn::Error::new(Span::call_site(), format!("Failed to get CARGO_PKG_NAME: {err}")))?
        .trim_start_matches(char::is_numeric)
        .chars()
        .map(|ch| if ch.is_alphanumeric() { ch } else { '_' })
        .collect::<String>();
    let minor_version: u8 = std::env::var("CARGO_PKG_VERSION_MINOR")
        .map_err(|err| syn::Error::new(Span::call_site(), format!("Failed to get CARGO_PKG_VERSION_MINOR: {err}")))?
        .parse()
        .expect("Failed to parse CARGO_PKG_VERSION_MINOR");
    let major_version: u8 = std::env::var("CARGO_PKG_VERSION_MAJOR")
        .map_err(|err| syn::Error::new(Span::call_site(), format!("Failed to get CARGO_PKG_VERSION_MAJOR: {err}")))?
        .parse()
        .expect("Failed to parse CARGO_PKG_VERSION_MAJOR");

    let register_impl = syn::parse2(quote! {
        impl qtbridge::qtbridge_runtime::QmlRegister for #struct_ident {
            const URI: &str = #uri;
            const ELEMENT_NAME: &str = #struct_name;
            const MINOR_VERSION: u8 = #minor_version;
            const MAJOR_VERSION: u8 = #major_version;
            const IS_SINGLETON: bool = #is_singleton;
        }
    })?;

    Ok(Some(QmlElementCode {
        register_fn,
        register_impl,
    }))
}
