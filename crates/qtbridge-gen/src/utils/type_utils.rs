// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

use crate::utils::type_to_string::{path_to_string_fallback, type_to_string, type_to_string_fallback};

#[derive(Copy, Clone, PartialEq)]
pub enum ValuePass {
    ByValue,
    ByConstReference,
    ByMutReference,
    // TODO: ByValueCopy ?
}

pub fn remove_ref_to_string(ty: &syn::Type) -> syn::Result<String> {
    type_to_string(remove_refs(ty))
}

/// Recursively unwraps the type until non-reference type is found.
pub fn remove_refs(ty: &syn::Type) -> &syn::Type {
    let mut unwrapped = ty;
    while let syn::Type::Reference(type_ref) = unwrapped {
        unwrapped = type_ref.elem.as_ref()
    }
    unwrapped
}

/// Remove one layer of reference if the type is a reference.
pub fn remove_ref(ty: &syn::Type) -> &syn::Type {
    if let syn::Type::Reference(type_ref) = ty {
        return type_ref.elem.as_ref()
    }
    ty
}

pub fn get_type_pass(ty: &syn::Type) -> ValuePass {
    match ty {
        syn::Type::Reference(reference) => {
            match &reference.mutability {
                Some(_) => ValuePass::ByMutReference,
                None => ValuePass::ByConstReference,
            }
        },
        _ => ValuePass::ByValue,
    }
}

pub fn get_take_value_code(value: &syn::Ident, pass: ValuePass) -> TokenStream {
    match pass {
        ValuePass::ByValue => quote!{ #value },
        ValuePass::ByConstReference => quote!{ &#value },
        ValuePass::ByMutReference => quote!{ &mut #value },
    }
}

/// Returns `true` if the input represents a reference type.
pub fn is_ref(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Reference(_))
}

pub fn is_mut_ref(ty: &syn::Type) -> bool {
    if let syn::Type::Reference(ref_) = ty {
        return ref_.mutability.is_some()
    }
    false
}

/// Extract `syn::Path` from `syn::Type` if it is `Path` variant.
pub fn path_from_type(src: &syn::Type) -> syn::Result<&syn::Path> {
    let syn::Type::Path(type_path) = src else {
        return Err(syn::Error::new(src.span(), format!("TypePath expected. Found type '{}'", type_to_string_fallback(src))))
    };
    if let Some(qself) = &type_path.qself {
        return Err(syn::Error::new(qself.span(), "Qualified self syntax is not supported"))
    }
    Ok(&type_path.path)
}

// Returns a reference to `syn::Ident` of the last path segment,
// or `None` if the path has no segments.
pub fn get_ident_of_last_path_segment(src: &syn::Path) -> Option<&syn::Ident> {
    src.segments.last()
        .map(|seg| &seg.ident)
}

// Returns a reference to `syn::Ident` of the last segment in the given `syn::Path`,
// or `syn::Error` if the path has no segments.
pub fn get_ident_of_last_path_segment_or_err(src: &syn::Path) -> syn::Result<&syn::Ident> {
    get_ident_of_last_path_segment(src)
        .ok_or_else(|| syn::Error::new(src.span(), format!("Failed to get the last segment from path '{}'", path_to_string_fallback(src))))
}
