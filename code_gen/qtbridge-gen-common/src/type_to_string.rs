// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::ToTokens;
use syn::spanned::Spanned;

pub fn type_to_string(src: &syn::Type) -> syn::Result<String> {
    match src {
        syn::Type::Array(type_array) =>
            type_array_to_string(type_array),
        syn::Type::BareFn(bare_fn) =>
            type_bare_fn_to_string(bare_fn),
        syn::Type::Path(type_path) =>
            type_path_to_string(type_path),
        syn::Type::Ptr(type_ptr) =>
            type_ptr_to_string(type_ptr),
        syn::Type::Reference(type_ref) =>
            type_ref_to_string(type_ref),
        syn::Type::Slice(type_slice) =>
            type_slice_to_string(type_slice),
        syn::Type::TraitObject(type_trait) =>
            type_trait_object_to_string(type_trait),
        syn::Type::Tuple(type_tuple) =>
            type_tuple_to_string(type_tuple),
        _ =>
            Err(syn::Error::new(src.span(), format!("Unsupported category '{:?}' of Rust type '{}'. Failed to convert to string", std::mem::discriminant(src), src.to_token_stream()))),
    }
}

pub fn type_to_string_fallback(src: &syn::Type) -> String {
    type_to_string(src)
        .unwrap_or_else(|_| src.to_token_stream().to_string())
}

pub fn type_array_to_string(src: &syn::TypeArray) -> syn::Result<String> {
    let elm_type = type_to_string(src.elem.as_ref())?;
    let len = expr_to_string(&src.len)?;
    Ok(format!("[{elm_type};{len}]"))
}

pub fn expr_to_string(src: &syn::Expr) -> syn::Result<String> {
    match src {
        syn::Expr::Path(expr_path) =>
            expr_path_to_string(expr_path),
        _ =>
            Err(syn::Error::new(src.span(), format!("Unsupported category '{:?}' of expression '{}'. Failed to convert to string", std::mem::discriminant(src), src.to_token_stream()))),
    }
}

pub fn expr_path_to_string(src: &syn::ExprPath) -> syn::Result<String> {
    check_qself_is_none(src.qself.as_ref())?;

    path_to_string(&src.path)
}

pub fn check_qself_is_none(src: Option<&syn::QSelf>) -> syn::Result<()> {
    if let Some(qself) = src {
        return Err(syn::Error::new(qself.span(), "QSelf is not supported in conversion to string"));
    }

    Ok(())
}

pub fn type_bare_fn_to_string(src: &syn::TypeBareFn) -> syn::Result<String> {
    if let Some(lt) = &src.lifetimes {
        return Err(syn::Error::new(lt.span(), "Bare function with explicit lifetimes are unsupported"))
    }
    if let Some(abi) = &src.abi {
        return Err(syn::Error::new(abi.span(), "Bare function with ABI are unsupported"))
    }
    if let Some(variadic) = &src.variadic {
        return Err(syn::Error::new(variadic.span(), "Bare function with variadic arguments are not supported"))
    }

    let maybe_unsafe = src.unsafety
        .map(|_| "unsafe ")
        .unwrap_or_default();
    let inputs = src.inputs.iter()
        .map(bare_fn_arg_to_string)
        .collect::<syn::Result<Vec<_>>>()?
        .join(", ");
    let output = return_type_to_string(&src.output)?;
    Ok(format!("{maybe_unsafe}fn ({inputs}){output}"))
}

pub fn bare_fn_arg_to_string(src: &syn::BareFnArg) -> syn::Result<String> {
    let mut result = String::new();
    if let Some((name, _colon)) = &src.name {
        result = format!("{}: ", name)
    }
    result.push_str(&type_to_string(&src.ty)?);

    Ok(result)
}

pub fn return_type_to_string(src: &syn::ReturnType) -> syn::Result<String> {
    let result = match src {
        syn::ReturnType::Default => "".into(),
        syn::ReturnType::Type(_, ty) => format!("-> {}", type_to_string(ty.as_ref())?),
    };
    Ok(result)
}

pub fn type_path_to_string(src: &syn::TypePath) -> syn::Result<String> {
    // We don't need to handle qualified self type
    // e.g. <Vec<T> as SomeTrait>::Associated
    // So ignore src.qself for now
    check_qself_is_none(src.qself.as_ref())?;

    path_to_string(&src.path)
}

pub(crate) fn type_ptr_to_string(src: &syn::TypePtr) -> syn::Result<String> {
    if src.const_token.is_some() == src.mutability.is_some() {
        return Err(syn::Error::new(src.span(), "Invalid combination of 'const' and 'mut' in pointer"));
    }

    let const_or_mut = if src.const_token.is_some() { "const" } else { "mut" };

    Ok(format!("*{const_or_mut} {}", type_to_string(src.elem.as_ref())?))
}

pub(crate) fn type_ref_to_string(src: &syn::TypeReference) -> syn::Result<String> {
    let maybe_mut = if src.mutability.is_some() { "mut " } else { "" };
    Ok(format!("&{maybe_mut}{}", type_to_string(src.elem.as_ref())?))
}

fn type_slice_to_string(src: &syn::TypeSlice) -> syn::Result<String> {
    let ty = type_to_string(src.elem.as_ref())?;
    Ok(format!("&[{ty}]"))
}

fn type_trait_object_to_string(src: &syn::TypeTraitObject) -> syn::Result<String> {
    let bounds_str = src.bounds.iter()
        .map(type_param_bound_to_string)
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(format!("dyn {}", bounds_str.join(" + ")))
}

fn type_param_bound_to_string(src: &syn::TypeParamBound) -> syn::Result<String> {
    match src {
        syn::TypeParamBound::Trait(trait_bound) => trait_bound_to_string(trait_bound),
        _ => Err(syn::Error::new(src.span(), format!("Unsupported kind '{:?}' of trait bound '{}'", std::mem::discriminant(src), src.to_token_stream())))
    }
}

fn trait_bound_to_string(src: &syn::TraitBound) -> syn::Result<String> {
    let modifier = match &src.modifier {
        syn::TraitBoundModifier::None => "",
        syn::TraitBoundModifier::Maybe(_) => "?",
    };
    let lifetimes = match &src.lifetimes {
        Some(lt) => bound_lifetimes_to_string(lt)?,
        None => String::new(),
    };
    let path = path_to_string(&src.path)?;
    let in_paren = format!("{modifier}{lifetimes} {path}");
    match &src.paren_token {
        Some(_) => Ok(format!("({in_paren})")),
        None => Ok(in_paren),
    }
}

fn bound_lifetimes_to_string(src: &syn::BoundLifetimes) -> syn::Result<String> {
    let lifetimes = src.lifetimes.iter()
        .map(generic_param_to_string)
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(format!("for <{}>", lifetimes.join(", ")))
}

fn generic_param_to_string(src: &syn::GenericParam) -> syn::Result<String> {
    Err(syn::Error::new(src.span(), format!("Unsupported kind '{:?}' of generic param '{}'", std::mem::discriminant(src), src.to_token_stream())))
}

fn type_tuple_to_string(src: &syn::TypeTuple) -> syn::Result<String> {
    let mut result = String::from('(');
    for elm in &src.elems {
        if result.len() > 1 {
            result.push_str(", ");
        }
        let elm_str = type_to_string(elm)?;
        result.push_str(&elm_str);
    }
    result.push(')');

    Ok(result)
}

pub fn path_to_string_fallback(src: &syn::Path) -> String {
    path_to_string(src)
        .unwrap_or_else(|_| src.to_token_stream().to_string())
}

pub fn path_to_string(src: &syn::Path) -> syn::Result<String> {
    // ignore src.leading_colon
    let segs = src.segments.iter()
        .map(path_segment_to_string)
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(segs.join("::"))
}

pub fn path_segment_to_string(src: &syn::PathSegment) -> syn::Result<String> {
    let ident = &src.ident;
    let args = match &src.arguments {
        syn::PathArguments::None => String::new(),
        syn::PathArguments::AngleBracketed(angle_bracketed) =>
            format!("<{}>", angle_bracketed_generic_arguments_to_string(angle_bracketed)?),
        syn::PathArguments::Parenthesized(parenthesized) =>
            format!("({})", parenthesized_generic_arguments_to_string(parenthesized)?),
    };
    Ok(format!("{ident}{args}"))
}

pub fn angle_bracketed_generic_arguments_to_string(src: &syn::AngleBracketedGenericArguments) -> syn::Result<String> {
    let args = src.args.iter()
        .map(generic_argument_to_string)
        .collect::<syn::Result<Vec<_>>>()?;
    Ok(args.join(", "))
}

pub(crate) fn generic_argument_to_string(src: &syn::GenericArgument) -> syn::Result<String> {
    match src {
        syn::GenericArgument::Type(ty) => type_to_string(ty),
        _ => Err(syn::Error::new(src.span(), format!("Unsupported variant of GenericArgument: {:?}", std::mem::discriminant(src)))),
    }
}

pub(crate) fn parenthesized_generic_arguments_to_string(src: &syn::ParenthesizedGenericArguments) -> syn::Result<String> {
    let inputs = src.inputs.iter()
        .map(type_to_string)
        .collect::<syn::Result<Vec<_>>>()?;
    let mut result = format!("({})", inputs.join(", "));
    let output = return_type_to_string(&src.output)?;
    if !output.is_empty() {
        result.push(' ');
        result.push_str(&output);
    }
    Ok(result)
}


pub fn path_to_ident_str(path: &syn::Path) -> syn::Result<String> {
    Ok(str_to_ident_str(path_to_string(path)?))
}

pub fn type_to_ident_str(ty: &syn::Type) -> syn::Result<String> {
    Ok(str_to_ident_str(type_to_string(ty)?))
}

fn str_to_ident_str(mut str: String) -> String {

    str = str.replace("&str", "string_slice");
    str = str.replace("&[", "slice_of_");
    str = str.replace('[', "array_of_");
    str = str.replace('(', "tuple_of_");

    str = str.replace("&", "ref ");
    str = str.replace("*", "ptr ");

    fn is_replace_needed(ch: char) -> bool {
        matches!(ch, ':' | '<' | '>' | ']' | ')' |'&' | ',' | ';' | ' ')
    }

    let mut result = String::with_capacity(str.len());
    let mut replaced_last = false;
    for ch in str.chars() {
        let need_replace = is_replace_needed(ch);
        if need_replace {
            if !replaced_last {
                result.push('_');
            }
        } else {
            result.push(ch);
        }
        replaced_last = need_replace;
    }

    result = result.trim_end_matches('_').to_owned();
    result
}
