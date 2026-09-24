// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::Naming;
use crate::signature_utils::{change_types_in_signature_to_monomorphed, substitute_qt_aliases_in_signature};


pub struct CppFunctionBridge {
    rust_name: Naming,
    sign: syn::Signature,
}

impl CppFunctionBridge {
    pub fn new(rust_name: Naming, mut sign: syn::Signature) -> syn::Result<Self> {
        change_types_in_signature_to_monomorphed(&mut sign)?;
        substitute_qt_aliases_in_signature(&mut sign)?;

        Ok(Self {
            rust_name,
            sign,
        })
    }
    pub fn signature(&self) -> &syn::Signature {
        &self.sign
    }
}

impl ToTokens for CppFunctionBridge {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self{ rust_name, sign } = self;
        let maybe_rust_name = (!rust_name.is_empty())
            .then(|| quote!{ #[rust_name = #rust_name] });

        quote! {
            #maybe_rust_name
            #sign;
        }.to_tokens(tokens);
    }
}


pub struct RustFunctionBridge {
    self_type: Option<Naming>,
    cxx_name: Naming,
    sign: syn::Signature
}

impl RustFunctionBridge {
    pub fn new(cxx_name: Naming, mut sign: syn::Signature) -> syn::Result<Self> {
        change_types_in_signature_to_monomorphed(&mut sign)?;

        Ok(Self {
            self_type: None,
            cxx_name,
            sign
        })
    }

    pub fn new_associated_function(self_type: Option<Naming>, cxx_name: Naming, mut sign: syn::Signature) -> syn::Result<Self> {
        change_types_in_signature_to_monomorphed(&mut sign)?;

        Ok(Self {
            self_type,
            cxx_name,
            sign,
        })
    }

    pub fn signature(&self) -> &syn::Signature {
        &self.sign
    }
}

impl ToTokens for RustFunctionBridge {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self{ self_type, cxx_name, sign } = self;
        let maybe_self = self_type.as_ref()
            .map(|self_type| quote!{ #[Self = #self_type] });
        let maybe_cxx_name = (!cxx_name.is_empty())
            .then(|| quote!{ #[cxx_name = #cxx_name] });

        quote! {
            #maybe_self
            #maybe_cxx_name
            #sign;
        }.to_tokens(tokens);
    }
}
