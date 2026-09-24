// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
use std::fmt::{Display, Result};

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident};

/// Helper for more convenient forming of names for functions/structures/modules/
/// in syn::ident/String format and interpolation into TokenStream/String with code.
#[derive(Clone)]
pub struct Naming {
    value: String,
}

impl Naming {
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    pub fn to_ident(&self) -> syn::Ident {
        format_ident!("{}", self.value)
    }

    pub fn to_path(&self) -> syn::Path {
        syn::parse_str(&self.value)
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to parse '{}' as syn::Path. Error: {err}", self.value)))
            .unwrap()
    }
}

impl ToTokens for Naming {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let as_tokens: TokenStream = syn::parse_str(&self.value)
            .map_err(|err| syn::Error::new(err.span(), format!("Failed to convert string '{}' to tokens. Error: {err}", self.value)))
            .unwrap();
        as_tokens.to_tokens(tokens);
    }
}

impl Display for Naming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        Display::fmt(&self.value, f)
    }
}

impl quote::IdentFragment for Naming {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result {
        Display::fmt(&self.value, f)
    }
}

impl From<&syn::Ident> for Naming {
    fn from(ident: &syn::Ident) -> Self {
        Self {
            value: ident.to_string()
        }
    }
}

impl From<String> for Naming {
    fn from(value: String) -> Self {
        Self {
            value
        }
    }
}

impl From<&str> for Naming {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_owned()
        }
    }
}


#[macro_export]
macro_rules! format_naming {
    ($($args:tt)*) => {
        Naming::from(format!($($args)*))
    };
}


pub mod rust {
    pub mod path {
        pub fn generated_module_dir(iface_name: &impl std::fmt::Display) -> String {
            let iface_module = super::module::from_struct_name(iface_name);
            format!("qtbridge-interfaces/src/generated/{iface_module}/")
        }
    }

    pub mod filename {
        pub fn proxy_rust() -> String {
            let module = super::module::proxy_rust();
            format!("{module}.rs")
        }

        pub fn proxy_rust_bridge() -> String {
            let module = super::module::proxy_rust_bridge();
            format!("{module}.rs")
        }

        pub fn proxy_cpp_bridge() -> String {
            let module = super::module::proxy_cpp_bridge();
            format!("{module}.rs")
        }

        pub fn vtable() -> String {
            let module = super::module::interface_trait();
            format!("{module}.rs")
        }

        pub fn type_gen_file_stem(module_name: &str) -> String {
            module_name.to_ascii_lowercase()
        }

        pub fn type_gen_file(module_name: &str) -> String {
            format!("{}.rs", type_gen_file_stem(module_name))
        }
    }

    pub mod module {
        use crate::Naming;

        pub fn from_struct_name(struct_name: &(impl std::fmt::Display + ?Sized)) -> Naming {
            crate::case_conv::camel_to_snake(&struct_name.to_string()).into()
        }

        pub fn interface_trait() -> Naming {
            "interface_trait".into()
        }

        pub fn proxy_cpp_bridge() -> Naming {
            "proxy_cpp_bridge".into()
        }

        pub fn proxy_rust() -> Naming {
            "proxy_rust".into()
        }

        pub fn proxy_rust_bridge() -> Naming {
            "proxy_rust_bridge".into()
        }
    }

    pub mod structure {
        use crate::Naming;

        pub fn proxy_rust(cpp_iface_name: &impl std::fmt::Display) -> Naming {
            format!("{cpp_iface_name}ProxyRust").into()
        }

        pub fn proxy_cpp(cpp_iface_name: &impl std::fmt::Display) -> Naming {
            format!("{cpp_iface_name}ProxyCpp").into()
        }

        pub fn proxy_cpp_in_camel_case(cpp_iface_name: &impl std::fmt::Display) -> Naming {
            format!("{}_proxy_cpp", crate::case_conv::camel_to_snake(&cpp_iface_name.to_string())).into()
        }
    }

    pub mod function {
        use crate::Naming;

        pub fn default(struct_name: &str) -> Naming {
            format!("{}_default", crate::case_conv::camel_to_snake(struct_name)).into()
        }

        pub fn drop(struct_name: &impl std::fmt::Display) -> Naming {
            format!("{}_drop", crate::case_conv::camel_to_snake(&struct_name.to_string())).into()
        }

        pub fn clone(struct_name: &str) -> Naming {
            format!("{}_clone", crate::case_conv::camel_to_snake(struct_name)).into()
        }

        pub fn eq(struct_name: &str) -> Naming {
            format!("{}_eq", crate::case_conv::camel_to_snake(struct_name)).into()
        }

        pub fn qmetatype(struct_name: &impl std::fmt::Display) -> Naming {
            crate::case_conv::camel_to_snake(&format!("{struct_name}Qmetatype")).into()
        }

        pub fn base(rust_name: &impl std::fmt::Display) -> Naming {
            format!("base_{rust_name}").into()
        }

        pub fn create_proxy_cpp(proxy_name: &impl std::fmt::Display) -> Naming {
            format!("create_{}", crate::case_conv::camel_to_snake(&proxy_name.to_string())).into()
        }

        pub fn create_proxy_cpp_at(proxy_name: &impl std::fmt::Display) -> Naming {
            format!("create_{}_at", crate::case_conv::camel_to_snake(&proxy_name.to_string())).into()
        }

        pub fn static_meta_object(proxy_name: &impl std::fmt::Display) -> Naming {
            format!("static_qmeta_object_of_{}", crate::case_conv::camel_to_snake(&proxy_name.to_string())).into()
        }

        pub fn sizeof_proxy_cpp(proxy_name: &impl std::fmt::Display) -> Naming {
            format!("size_of_{}", crate::case_conv::camel_to_snake(&proxy_name.to_string())).into()
        }

        pub fn alignof_proxy_cpp(proxy_name: &impl std::fmt::Display) -> Naming {
            format!("align_of_{}", crate::case_conv::camel_to_snake(&proxy_name.to_string())).into()
        }

        pub fn qmetatype_list_of_proxy_cpp(proxy_name: &impl std::fmt::Display) -> Naming {
            format!("qmetatype_list_of_{}", crate::case_conv::camel_to_snake(&proxy_name.to_string())).into()
        }

        pub fn drop_self() -> Naming {
            "drop_self".into()
        }

        pub fn get_cpp_proxy() -> Naming {
            "get_cpp_proxy".into()
        }

        pub fn get_cpp_proxy_mut() -> Naming {
            "get_cpp_proxy_mut".into()
        }
    }

    pub mod constant {
    }
}

pub mod cpp {
    pub mod path {
        pub fn proxy_header(iface_name: &impl std::fmt::Display) -> String {
            let dir = super::super::rust::path::generated_module_dir(iface_name);
            let filename = super::filename::proxy_header(iface_name);
            format!("{dir}cpp/{filename}")
        }
    }

    pub mod filename {
        pub fn proxy_header(iface_name: &impl std::fmt::Display) -> String {
            format!("{}.h", super::class::proxy_cpp(iface_name))
        }

        pub fn proxy_cpp(iface_name: &impl std::fmt::Display) -> String {
            format!("{}.cpp", super::class::proxy_cpp(iface_name))
        }

        pub fn type_gen_header(module_name: &str) -> String {
            format!("{}.h", module_name.to_ascii_lowercase())
        }

        pub fn type_gen_cpp(module_name: &str) -> String {
            format!("{}.cpp", module_name.to_ascii_lowercase())
        }
    }

    pub mod namespace {
        pub const fn bridge() -> &'static str {
            "rust::bridge"
        }

        pub fn type_bridge(submod_name: &str) -> String {
            format!("rust::bridge::{}", crate::case_conv::camel_to_snake(submod_name))
        }
    }

    pub mod class {
        use crate::Naming;

        pub fn proxy_cpp(cpp_iface_name: &impl std::fmt::Display) -> Naming {
            format!("{cpp_iface_name}ProxyCpp").into()
        }

        pub fn proxy_rust(cpp_iface_name: &str) -> Naming {
            format!("{cpp_iface_name}ProxyRust").into()
        }

    }

    pub mod class_variables {

        pub mod proxy {
            use crate::Naming;

            pub fn rust_proxy() -> Naming {
                "m_rustProxy".into()
            }

            pub fn rust_obj() -> Naming {
                "m_rustObj".into()
            }
        }
    }

    pub mod function {
        use crate::Naming;

        pub fn default(struct_name: &str) -> Naming {
            format!("{struct_name}_Default").into()
        }

        pub fn drop(struct_name: &str) -> Naming {
            format!("{struct_name}_Drop").into()
        }

        pub fn clone(struct_name: &str) -> Naming {
            format!("{struct_name}_Clone").into()
        }

        pub fn eq(struct_name: &str) -> Naming {
            format!("{struct_name}_Eq").into()
        }

        pub fn qmetatype(struct_name: &impl std::fmt::Display) -> Naming {
            format!("{struct_name}_QMetaType").into()
        }

        pub fn base(cpp_name: &str) -> Naming {
            format!("base_{cpp_name}").into()
        }

        pub fn drop_self() -> Naming {
            "dropSelf".into()
        }

        pub fn create_proxy_cpp(proxy_name: &impl std::fmt::Display) -> Naming {
            format!("create_{proxy_name}").into()
        }

        pub fn create_proxy_cpp_at(proxy_name: &impl std::fmt::Display) -> Naming {
            format!("create_{proxy_name}_At").into()
        }

        pub fn static_meta_object(proxy_name: &impl std::fmt::Display) -> Naming {
            format!("staticQMetaObjectOf_{proxy_name}").into()
        }

        pub fn sizeof_proxy_cpp(proxy_name: &impl std::fmt::Display) -> Naming {
            format!("sizeOf_{proxy_name}").into()
        }

        pub fn alignof_proxy_cpp(proxy_name: &impl std::fmt::Display) -> Naming {
            format!("alignOf_{proxy_name}").into()
        }

        pub fn qmetatype_list(proxy_name: &impl std::fmt::Display) -> Naming {
            format!("qmetaTypeListOf_{proxy_name}").into()
        }

        pub fn inline_function_prefix() -> Naming {
            "inlineCppFn".into()
        }
    }
}

