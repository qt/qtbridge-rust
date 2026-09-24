// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::case_conv;
use crate::signature_utils::{check_signature, get_return_type, get_typed_arg_ident};
use crate::type_to_cpp::type_to_cpp;
use crate::type_to_string::type_to_string_fallback;

pub struct CppFnArg {
    rust_type: syn::Type,
    name: String,
}

impl CppFnArg {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl TryFrom<&syn::FnArg> for CppFnArg {
    type Error = syn::Error;

    fn try_from(value: &syn::FnArg) -> syn::Result<Self> {
        match value {
            syn::FnArg::Receiver(receiver) => Self::try_from(receiver),
            syn::FnArg::Typed(pat_type) => Self::try_from(pat_type),
        }
    }
}

impl TryFrom<&syn::Receiver> for CppFnArg {
    type Error = syn::Error;

    fn try_from(value: &syn::Receiver) -> syn::Result<Self> {
        Ok(Self {
            rust_type: value.ty.as_ref().clone(),
            name: "self".to_owned(),
        })
    }
}

impl TryFrom<&syn::PatType> for CppFnArg {
    type Error = syn::Error;

    fn try_from(value: &syn::PatType) -> syn::Result<Self> {
        Ok(Self{
            rust_type: value.ty.as_ref().clone(),
            name: get_typed_arg_ident(value)?.to_string()
        })
    }
}

impl std::fmt::Display for CppFnArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cpp_type = type_to_cpp(&self.rust_type)
            .map_err(|_err| std::fmt::Error{})?;
        let name = self.get_name();

        write!(f, "{cpp_type} {name}")
    }
}


pub struct CppFnSign {
    return_type: Option<String>,
    name: String,
    this: Option<CppFnArg>,
    typed_arguments: Vec<CppFnArg>,
}

impl CppFnSign {
    pub fn new_from_rust_sig(sig: &syn::Signature, cpp_name: Option<String>) -> syn::Result<Self> {
        check_signature(sig)?;

        let return_type = get_cpp_return_type(&sig.output)?;

        let name = cpp_name.unwrap_or_else(||case_conv::snake_to_camel(&sig.ident.to_string()));

        let mut this = None;
        let mut src_typed_args = sig.inputs.iter();
        if let Some(receiver) = sig.receiver() {
            this = Some(CppFnArg::try_from(receiver)?);
            src_typed_args.next();
        }

        let typed_arguments = src_typed_args
            .map(CppFnArg::try_from)
            .collect::<syn::Result<_>>()?;

        Ok(Self{
            return_type,
            name,
            this,
            typed_arguments,
        })

    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_return_type(&self) -> &str {
        self.return_type
            .as_ref()
            .map_or("void", |type_str| type_str.as_str() )
    }

    pub fn get_maybe_self_argument(&self) -> String {
        if let Some(this) = &self.this {
            return this.to_string();
        }

        String::new()
    }

    pub fn get_typed_argument_list(&self) -> String {
        self.typed_arguments.iter()
            .fold(String::new(), |acc, arg| {
                if acc.is_empty() {
                    arg.to_string()
                }
                else {
                    format!("{acc}, {arg}")
                }
            })
    }

    pub fn get_typed_arguments_forward(&self) -> String {
        self.typed_arguments.iter()
            //.map(|arg| format!("std::forward<{}>({})", arg.get_type_str(), arg.get_name())) // add std::forward for every argument
            .map(CppFnArg::get_name)
            .collect::<Vec<_>>().join(", ")
    }

    pub fn get_maybe_return_op(&self) -> &'static str {
        if self.return_type.is_some() { "return "} else { "" }
    }

    pub fn to_declaration_string(&self, include_self: bool) -> String {
        let return_type = self.get_return_type();
        let name        = self.get_name();
        let typed_args  = self.get_typed_argument_list();
        let maybe_self  = if include_self {
            let mut s = self.get_maybe_self_argument();
            if !s.is_empty() && !typed_args.is_empty() {
                s.push_str(", ");
            }
            s
        }
        else {
            String::new()
        };
        format!("{return_type} {name}({maybe_self}{typed_args})")
    }

}

fn get_cpp_return_type(return_type: &syn::ReturnType) -> syn::Result<Option<String>> {
    let Some(ty) = get_return_type(return_type) else {
        return Ok(None)
    };

    let cpp = type_to_cpp(ty)
        .map_err(|err| syn::Error::new(err.span(), format!("Return type '{}' is currently unsupported by the bridge. Error: {err}",
            type_to_string_fallback(ty))))?;
    Ok(Some(cpp))
}
