// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::ToTokens;
use syn::spanned::Spanned;

pub fn get_typed_arg_type(arg: &syn::FnArg) -> Option<&syn::Type> {
    get_typed_arg(arg)
        .map(|pat| pat.ty.as_ref())
}

pub fn get_typed_arg(arg: &syn::FnArg) -> Option<&syn::PatType> {
    match arg {
        syn::FnArg::Receiver(_) => None,
        syn::FnArg::Typed(pat_type) => Some(pat_type),
    }
}

/// Takes a function signature and returns an iterator over its typed arguments,
/// skipping the `Self` receiver.
pub fn get_typed_args(sign: &syn::Signature) -> impl Iterator<Item = &syn::PatType> {
    sign.inputs.iter()
        .filter_map(get_typed_arg)
}

/// Takes a function signature and returns an iterator over the types of arguments
/// skipping the `Self` receiver type.
pub fn get_typed_args_types(sign: &syn::Signature) -> impl Iterator<Item = &syn::Type> {
    get_typed_args(sign)
        .map(|arg| arg.ty.as_ref())
}

pub fn is_arg_self_ref(arg: &syn::FnArg, expected_mut: Option<bool>) -> bool {
    let syn::FnArg::Receiver(receiver) = arg else {
        return false;
    };

    let syn::Type::Reference(_) = receiver.ty.as_ref() else {
        return false;
    };

    let Some(expected_mut) = expected_mut else {
        return true;
    };

    expected_mut == receiver.mutability.is_some()
}

pub fn is_self_mut(sig: &syn::Signature) -> bool {
    sig.inputs.first()
        .is_some_and(|self_arg| is_arg_self_ref(self_arg, Some(true)))
}

pub fn get_return_type(return_type: &syn::ReturnType) -> Option<&syn::Type> {
    match return_type {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_rarrow, ty) => Some(ty.as_ref()),
    }
}

pub fn get_typed_arg_ident(arg: &syn::PatType) -> syn::Result<syn::Ident> {
    let syn::Pat::Ident(pat_ident) = arg.pat.as_ref() else {
        return Err(syn::Error::new(arg.span(), format!("Failed to get argument name from {}", arg.pat.to_token_stream())));
    };

    Ok(pat_ident.ident.clone())
}
