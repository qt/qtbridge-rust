// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;

use crate::case_conv;
use crate::qt_alias_mapping::QtAliasMapping;
use crate::qt_generic_mapping::QtGenericMapping;
use crate::type_to_cpp::is_type_mapped_to_cpp;
use crate::type_to_string::type_to_string_fallback;
use crate::type_utils::is_ptr;


pub fn check_signature(sign: &syn::Signature) -> syn::Result<()> {
    for typed_arg in get_typed_args(sign) {
        let arg_type = typed_arg.ty.as_ref();
        if !is_type_mapped_to_cpp(arg_type) {
            return Err(syn::Error::new(typed_arg.ty.span(), format!("Type '{}' of argument is not supported by bridge", type_to_string_fallback(arg_type))));
        }
    }

    Ok(())
}


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

pub fn change_first_arg<P: Default>(new_arg: syn::FnArg, src: &Punctuated<syn::FnArg, P>) -> Punctuated<syn::FnArg, P> {
    let mut args = Punctuated::new();
    args.push(new_arg);
    args.extend(src.iter()
        .skip(1)
        .cloned());
    args
}

pub fn get_return_type(return_type: &syn::ReturnType) -> Option<&syn::Type> {
    match return_type {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_rarrow, ty) => Some(ty.as_ref()),
    }
}

pub fn get_arg_ident(arg: &syn::FnArg) -> syn::Result<syn::Ident> {
    match arg {
        syn::FnArg::Receiver(receiver) => Ok(get_receiver_arg_ident(receiver)),
        syn::FnArg::Typed(pat_type) => get_typed_arg_ident(pat_type),
    }
}

pub fn get_ident_in_snake_case(src: &syn::Ident) -> syn::Ident {
    let name_in_cc = case_conv::camel_to_snake(&src.to_string());
    if *src == name_in_cc {
        src.clone()
    }
    else {
        syn::Ident::new(&name_in_cc, src.span())
    }
}

pub(crate) fn get_receiver_arg_ident(arg: &syn::Receiver) -> syn::Ident {
    arg.self_token.into()
}

pub fn get_typed_arg_ident(arg: &syn::PatType) -> syn::Result<syn::Ident> {
    let syn::Pat::Ident(pat_ident) = arg.pat.as_ref() else {
        return Err(syn::Error::new(arg.span(), format!("Failed to get argument name from {}", arg.pat.to_token_stream())));
    };

    Ok(pat_ident.ident.clone())
}

pub fn change_arg_ident_to_camel_case<P: Default>(src: &Punctuated<syn::FnArg, P>) -> syn::Result<Punctuated<syn::FnArg, P>> {
    let mut result = Punctuated::new();
    for arg in src.iter() {
        let new_arg = match arg {
            syn::FnArg::Receiver(receiver) => receiver.clone().into(),
            syn::FnArg::Typed(pat_type) => {
                match pat_type.pat.as_ref() {
                    syn::Pat::Ident(pat_ident) => {
                        let new_pat_ident = syn::PatIdent {
                            ident: get_ident_in_snake_case(&pat_ident.ident),
                            ..pat_ident.clone()
                        };
                        let new_pat_type = syn::PatType {
                            pat: Box::new(new_pat_ident.into()),
                            ..pat_type.clone()
                        };
                        new_pat_type.into()
                    },
                    _ => return Err(syn::Error::new(pat_type.span(), format!("Failed to get argument name from '{}'", pat_type.to_token_stream())))
                }
            },
        };
        result.push(new_arg);
    }

    Ok(result)
}

pub fn change_arg_idents_in_signature_to_camel_case(src: &syn::Signature) -> syn::Result<syn::Signature> {
    Ok(syn::Signature {
        inputs: change_arg_ident_to_camel_case(&src.inputs)?,
        ..src.clone()
    })
}

pub fn change_types_in_signature_to_monomorphed(src: &mut syn::Signature) -> syn::Result<()> {
    QtGenericMapping::map_signature(src)
}

pub fn signature_eq(sign: &syn::Signature, reference: &syn::Signature) -> syn::Result<()> {
    let args = &sign.inputs;

    let args_ref = &reference.inputs;
    if args.len() != args_ref.len() {
        if args.len() < args_ref.len() {
            return Err(syn::Error::new(args.span(), format!("Too few function arguments ({} instead of {})", args.len(), args_ref.len())));
        }
        return Err(syn::Error::new(args[args_ref.len()].span(), format!("Too many function arguments ({} instead of {})", args.len(), args_ref.len())));
    }

    for idx in 1..args.len() {
        let syn::FnArg::Typed(lhs) = &args[idx] else {
            return Err(syn::Error::new(args[idx].span(), "Method argument is not typed"));
        };

        let syn::FnArg::Typed(rhs) = &args_ref[idx] else {
            return Err(syn::Error::new(args_ref[idx].span(), "Method argument is not typed in the interface declaration"));
        };

        if *lhs.ty != *rhs.ty {
            let lhs_str = lhs.ty.to_token_stream().to_string();
            let rhs_str = rhs.ty.to_token_stream().to_string();
            return Err(syn::Error::new(lhs.ty.span(), format!("Type of a method argument does not match interface ({:?} vs {:?})", lhs_str, rhs_str)));
        }
    }

    if sign.output != reference.output {
        let span = if let syn::ReturnType::Type(_arrow, ty) = &sign.output {
            ty.span()
        } else {
            sign.ident.span()
        };
        return Err(syn::Error::new(span, format!("Method return type does not match interface ({:?} vs {:?})", return_type_to_str(&sign.output), return_type_to_str(&reference.output))));
    }

    Ok(())
}

fn return_type_to_str(return_ty: &syn::ReturnType) -> String {
    match return_ty {
        syn::ReturnType::Default => "()".into(),
        syn::ReturnType::Type(_arrow, ty) => ty.to_token_stream().to_string(),
    }
}

pub fn substitute_qt_aliases_in_signature(src: &mut syn::Signature) -> syn::Result<()> {
    let mut map = QtAliasMapping::new();

    let inputs = src.inputs.iter()
        .cloned()
        .map(|mut arg| {
            map.visit_fn_arg_mut(&mut arg);
            arg
        })
        .collect();

    let mut output = src.output
        .clone();
    map.visit_return_type_mut(&mut output);

    map.result()?;

    src.inputs = inputs;
    src.output = output;

    Ok(())
}

pub fn is_unsafe(sign: &syn::Signature) -> bool {
    get_typed_args_types(sign)
        .chain(get_return_type(&sign.output))
        .any(is_ptr)
}
