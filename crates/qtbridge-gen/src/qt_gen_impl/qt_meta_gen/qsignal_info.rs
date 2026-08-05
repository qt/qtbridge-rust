// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitStr};
use syn::spanned::Spanned;

use qtbridge_gen_common::case_conv;
use crate::function_with_attributes::{FunctionWithAttributes, BlockOrSemi};
use qtbridge_gen_common::parse_utils::{parse_name_value, partition_attr_by};
use qtbridge_gen_common::signature_utils::{get_typed_args, get_typed_args_types, is_self_mut};
use qtbridge_gen_common::type_utils::remove_refs;
use crate::meta_call_check::check_meta_call_signature;
use crate::qt_gen_impl::qt_meta_gen;
use crate::qt_gen_impl::qobject_macro_params::QObjectMacroParams;
use qt_meta_gen::meta_call_bridge_generator::MetaCallBridgeGenerator;
use qt_meta_gen::traits::{ExpandTokens, QmlName};

#[derive(Default)]
struct QSignalMetaParams {
    name: Option<syn::LitStr>,
}

pub struct QSignalInfo {
    attrs: Vec<syn::Attribute>,     // Attributes other than qsignal
    meta_params: QSignalMetaParams, // Params extracted from qsignal attribute
    vis: syn::Visibility,
    sig: syn::Signature,
    global_options: QObjectMacroParams,
}

impl QSignalInfo {
    pub fn new(input: FunctionWithAttributes, global_options: QObjectMacroParams) -> syn::Result<Self> {
        Self::check_signature(&input.sig)?;

        let (attrs, signal_attr) = partition_attr_by(input.attrs.clone(), Self::is_for_me);
        let signal = signal_attr
            .ok_or_else(|| syn::Error::new(input.sig.span(), "qsignal attribute was not found for the function"))?;

        if let BlockOrSemi::Block(block) = input.block
            && !block.stmts.is_empty() {
                return Err(syn::Error::new(block.span(), "qsignal must not contain any code in brackets"));
            }

        let meta_params = get_qsignal_meta_params(&signal)?;

        Ok(QSignalInfo{
            attrs,
            meta_params,
            vis: input.vis,
            sig: input.sig,
            global_options,
        })
    }

    pub fn is_mut(&self) -> bool {
        is_self_mut(&self.sig)
    }

    pub fn is_for_me(attr: &syn::Attribute) -> bool {
        attr.style == syn::AttrStyle::Outer && attr.path().is_ident("qsignal")
    }

    pub fn get_rust_name(&self) -> syn::Ident {
        self.sig.ident.clone()
    }

    /// Get count of arguments after &self
    pub fn get_typed_arg_count(&self) -> usize {
        get_typed_args(&self.sig).count()
    }

    pub fn get_arg_type(&self, num: usize) -> syn::Result<&syn::Type> {
        get_typed_args_types(&self.sig)
            .nth(num)
            .ok_or_else(|| syn::Error::new(self.sig.span(), format!("Failed to get typed argument #{num}")))
    }

    pub fn get_meta_registration_code(&self) -> syn::Result<TokenStream> {
        let sig = &self.sig;

        let name = self.get_qml_name_span().0;
        let arg_types_no_ref: Vec<syn::Type> = get_typed_args_types(sig)
            .map(|ty| remove_refs(ty).clone())
            .collect();

        let register_signal = quote!{
            meta_obj.as_mut().register_signal(
                #name,
                &[#(<#arg_types_no_ref as QMetaCallArg>::wire_metatype()),*]);
        };
        Ok(register_signal)
    }

    fn check_signature(sign: &syn::Signature) -> syn::Result<()> {
        check_meta_call_signature(sign)
    }
}

mod qsignal_keywords {
    syn::custom_keyword!(qml_name);
}

fn get_qsignal_meta_params(attr: &syn::Attribute) -> syn::Result<QSignalMetaParams> {
    match &attr.meta {
        syn::Meta::Path(_) => Ok(QSignalMetaParams::default()),
        syn::Meta::List(meta_list) => match syn::parse2::<QSignalMetaParams>(meta_list.tokens.clone()) {
            Ok(params) => Ok(params),
            Err(err) => Err(syn::Error::new(err.span(), format!("Failed to parse qsignal attributes: {}", err))),
        },
        _ => Err(syn::Error::new(attr.span(), "Unexpected format of qsignal attributes"))
    }
}

impl syn::parse::Parse for QSignalMetaParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;

        while !input.is_empty() {
            if input.peek(qsignal_keywords::qml_name) {
                name = Some(parse_name_value::<Ident, LitStr>(input)?.1);
                // TODO: check charset of name?
            }
            else {
                return Err(input.error("Unsupported qsignal parameter attribute"));
            }
        }

        Ok(Self{
            name,
        })
    }
}

impl QmlName for QSignalInfo {
    fn get_qml_name_span(&self) -> (String, proc_macro2::Span) {
        if let Some(name) = self.meta_params.name.as_ref() {
            (name.value(), name.span())
        }
        else if self.global_options.convert_to_camel_case {
            let ident = &self.sig.ident;
            (case_conv::snake_to_camel(&ident.to_string()), ident.span())
        }
        else {
            (self.sig.ident.to_string(), self.sig.ident.span())
        }
    }
}

impl ExpandTokens for QSignalInfo {
    fn expand_tokens(&self) -> syn::Result<TokenStream> {
        let Self { attrs, vis, sig, .. } = self;
        let bridge_generator = MetaCallBridgeGenerator::new(sig);
        let qml_name = self.get_qml_name_span().0;
        let argv_setup = bridge_generator.generate_argv_setup_for_signals()?;
        if !self.is_mut() {
            let err_span = sig.receiver()
                .map_or_else(|| sig.ident.span(), |r| r.self_token.span());
            return Err(syn::Error::new(err_span,
                       "The function signature of a signal has to be mutable."))
        }
        let code = quote! {
            #(#attrs)*
            #vis
            #sig
            {
                use qtbridge::qtbridge_runtime::QMetaCallArg;
                let proxy = <Self as qtbridge::QObjectHolder>::try_get_rust_proxy_ptr(self).expect("No proxy");
                #argv_setup
                qtbridge::qtbridge_runtime::qproxies::QRustProxy::emit_signal(unsafe { &*proxy }, self, #qml_name, argv.as_slice())
            }
        };
        Ok(code)
    }
}
