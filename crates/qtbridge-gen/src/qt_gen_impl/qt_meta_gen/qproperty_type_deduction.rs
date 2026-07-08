// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::ToTokens;
use syn::spanned::Spanned;

use qtbridge_gen_common::signature_utils::{get_return_type, get_typed_arg_type, is_arg_self_ref};
use crate::meta_call_check::check_meta_call_type;


/// Deduces a property's type from its getter function.
///
/// # Arguments
///
/// * `getter_ident` - The ident of the getter function.
/// * `methods` - The signatures of the given structure functions.
pub(crate) fn deduce_type_from_getter<'a>(getter_ident: &syn::Ident, methods: &'a [syn::Signature]) -> syn::Result<&'a syn::Type> {
    let getter = methods.iter()
        .find(|g| g.ident == *getter_ident)
        .ok_or_else(|| syn::Error::new(getter_ident.span(), format!("Property getter '{getter_ident}' not found")))?;

    get_property_getter_type(getter)
        .map_err(|err| syn::Error::new(err.span(), format!("Function '{getter_ident}' is not suitable to be property getter.\nReason: {err}")))
}

/// Deduces a property's type from its setter function.
///
/// # Arguments
///
/// * `setter_ident` - The ident of the setter function.
/// * `methods` - The signatures of the given structure functions.
pub(crate)fn deduce_type_from_setter<'a>(setter_ident: &syn::Ident, methods: &'a [syn::Signature]) -> syn::Result<&'a syn::Type> {
    let setter = methods.iter()
        .find(|s| s.ident == *setter_ident)
        .ok_or_else(||syn::Error::new(setter_ident.span(), format!("Property setter '{setter_ident}' not found")))?;

    get_property_setter_type(setter)
        .map_err(|err| syn::Error::new(err.span(), format!("Function '{setter_ident}' is not suitable to be property setter.\nReason: {err}")))
}

/// Deduces a property's type from the type of a struct field.
///
/// # Arguments
///
/// * `field_ident` -  The identifier of the field within the struct.
/// * `fields` - The fields of the struct, given as an iterator.
pub(crate) fn deduce_type_from_member<'a>(field_ident: &syn::Ident, mut fields: impl Iterator<Item = &'a syn::Field>) -> syn::Result<&'a syn::Type> {
    let field = fields
        .find(|f| f.ident.as_ref().is_some_and(|ident| ident == field_ident))
        .ok_or_else(|| syn::Error::new(field_ident.span(), format!("Field '{field_ident}' not found")))?;

    Ok(&field.ty)
}

fn get_property_getter_type(sig: &syn::Signature) -> syn::Result<&syn::Type> {
    let args = &sig.inputs;
    if args.len() != 1 || !is_arg_self_ref(&args[0], Some(false)) {
        return Err(syn::Error::new(sig.span(), "Property getter must have single argument (&self)"));
    }

    let return_type = get_return_type(&sig.output)
        .ok_or_else(|| syn::Error::new(sig.span(), format!("Getter has return type not specified : {}", sig.to_token_stream())))?;

    check_meta_call_type(return_type)?;
    Ok(return_type)
}

fn get_property_setter_type(sig: &syn::Signature) -> syn::Result<&syn::Type> {
    let args = &sig.inputs;
    if args.len() != 2 {
        let span = match args.len() {
            0 => sig.ident.span(),
            1 => args[0].span(),
            _ => args[2].span(),
        };
        return Err(syn::Error::new(span, "Property setter supposed to have 2 arguments (&self and value)"));
    }

    let arg0 = &args[0];
    if !is_arg_self_ref(arg0, None) {
        return Err(syn::Error::new(arg0.span(), "First argument must be &self"));
    }

    let arg1 = &args[1];
    let arg_type = get_typed_arg_type(arg1)
        .ok_or_else(|| syn::Error::new(arg1.span(), format!("Failed to get type of argument: '{}'", arg1.to_token_stream())))?;

    check_meta_call_type(arg_type)?;
    Ok(arg_type)
}
