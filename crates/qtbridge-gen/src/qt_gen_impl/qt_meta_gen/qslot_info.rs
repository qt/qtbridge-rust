// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{spanned::Spanned, Ident, LitStr};
use qtbridge_gen_common::case_conv;
use crate::function_with_attributes::{BlockOrSemi, FunctionWithAttributes};
use qtbridge_gen_common::parse_utils::{parse_name_value, partition_attr_by};
use qtbridge_gen_common::signature_utils::is_self_mut;
use crate::meta_call_check::check_meta_call_signature;

use crate::qt_gen_impl::qt_meta_gen;
use crate::qt_gen_impl::qobject_macro_params::QObjectMacroParams;
use qt_meta_gen::meta_call_bridge_generator;
use meta_call_bridge_generator::MetaCallBridgeGenerator;
use qt_meta_gen::traits::{ExpandTokens, QmlName};

#[derive(Default)]
struct QSlotMetaParams {
    name: Option<syn::LitStr>,
}

pub struct QSlotInfo {
    id: u32,                      // Id that will be used to identify a slot in the handler function.
    meta_params: QSlotMetaParams, // Params extracted from qslot attribute
    func: syn::ImplItemFn,        // Slot function
    global_options: QObjectMacroParams,
}

impl QSlotInfo {
    pub fn new(input: FunctionWithAttributes, id: u32, global_options: QObjectMacroParams) -> syn::Result<Self> {
        Self::check_signature(&input.sig)?;

        let (attrs, slot_attr) = partition_attr_by(input.attrs.clone(), Self::is_for_me);
        let slot_attr = slot_attr
            .ok_or_else(|| syn::Error::new(input.sig.span(), "qslot attribute was not found for the function"))?;

        let BlockOrSemi::Block(block) = input.block else {
            return Err(syn::Error::new(input.sig.span(), "qslot must contain brackets (with some code optionally)"));
        };

        let meta_params = get_qslot_meta_params(&slot_attr)?;

        let func = syn::ImplItemFn {
            attrs,
            vis: input.vis,
            defaultness: None,
            sig: input.sig,
            block,
        };

        Ok(QSlotInfo {
            id,
            meta_params,
            func,
            global_options,
        })
    }

    pub fn is_for_me(attr: &syn::Attribute) -> bool {
        attr.style == syn::AttrStyle::Outer && attr.path().is_ident("qslot")
    }

    pub fn is_mut(&self) -> bool {
        is_self_mut(&self.func.sig)
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    /// Generate code for registration of the given slot in `DynamicMetaObjectBuilder`.
    ///
    /// For a slot defined as:
    /// ```ignore
    /// #[qslot]
    /// pub fn do_something(&mut self, i: i32, s: &String) -> String {
    ///     ...
    /// }
    /// ```
    ///
    /// the generated output would be:
    /// ```ignore
    /// meta_obj.as_mut().register_slot(
    ///     "doSomething",
    ///     &[<i32 as QMetaCallArg>::wire_metatype(), <String as QMetaCallArg>::wire_metatype()],
    ///     &<String as QMetaCallArg>::wire_metatype());
    /// ```
    pub fn get_meta_registration_code(&self) -> syn::Result<TokenStream> {
        let name = self.get_qml_name_span().0;
        let id = &self.id;
        let sig = &self.func.sig;

        let bridge_generator = MetaCallBridgeGenerator::new(sig)?;
        let input_meta_types = bridge_generator.get_input_metatypes()
            .collect::<Vec<_>>();
        let output_meta_types = bridge_generator.get_output_metatype();

        let result_meta_type = output_meta_types
            .map(|ty| quote!{ &<#ty as QMetaCallArg>::wire_metatype() })
            .unwrap_or_else(|| quote! { &qtbridge::qtbridge_type_lib::QMetaType::default() });

        let mutability = match self.is_mut() {
            true => quote! { Mutable },
            false => quote! { Immutable },
        };

        let register_slot = quote! {
            meta_obj.as_mut().register_slot(
                #name,
                #id,
                 &[#(<#input_meta_types as QMetaCallArg>::wire_metatype()),*],
                #result_meta_type,
                qtbridge::qtbridge_runtime::dynamicmetaobjectbuilder::Mutability::#mutability);
        };

        Ok(register_slot)
    }

    /// Generate code that goes to the dedicated arm of `match` operator in DispatchMetaCall::invoke_slot().
    pub fn get_invoke_code(&self) -> syn::Result<TokenStream> {
        let sig = &self.func.sig;
        let method_ident = &sig.ident;

        let bridge_generator = MetaCallBridgeGenerator::new(sig)?;

        let fn_call = syn::parse_quote! {
            self.#method_ident()
        };
        bridge_generator.generate_bridge_metacall_to_user_fn(fn_call)
    }

    fn check_signature(sign: &syn::Signature) -> syn::Result<()> {
        check_meta_call_signature(sign)
    }
}

fn get_qslot_meta_params(attr: &syn::Attribute) -> syn::Result<QSlotMetaParams> {
    match &attr.meta {
        syn::Meta::Path(_) => Ok(QSlotMetaParams::default()),
        syn::Meta::List(meta_list) => match syn::parse2::<QSlotMetaParams>(meta_list.tokens.clone()) {
            Ok(params) => Ok(params),
            Err(err) => Err(syn::Error::new(err.span(),format!("Failed to parse qslot attribute parameters: {}", err))),
        },
        _ => Err(syn::Error::new(attr.meta.span(), "Unexpected format of qslot attributes"))
    }
}


mod qslot_keywords {
    syn::custom_keyword!(qml_name);
}

impl syn::parse::Parse for QSlotMetaParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;

        while !input.is_empty() {
            if input.peek(qslot_keywords::qml_name) {
                name = Some(parse_name_value::<Ident, LitStr>(input)?.1);
                // TODO: check charset of name?
            }
            else {
                return Err(input.error("Unsupported qslot parameter attribute"));
            }
        }

        Ok(Self{
            name,
        })
    }
}

impl QmlName for QSlotInfo {
    fn get_qml_name_span(&self) -> (String, proc_macro2::Span) {
        if let Some(name) = self.meta_params.name.as_ref() {
            (name.value(), name.span())
        }
        else if self.global_options.convert_to_camel_case {
            let ident = &self.func.sig.ident;
            (case_conv::snake_to_camel(&ident.to_string()), ident.span())
        }
        else {
            (self.func.sig.ident.to_string(), self.func.sig.ident.span())
        }
    }
}

impl ExpandTokens for QSlotInfo {
    fn expand_tokens(&self) -> syn::Result<TokenStream> {
        Ok(self.func.to_token_stream())
    }
}
