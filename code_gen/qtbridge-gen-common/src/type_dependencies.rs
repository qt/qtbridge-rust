// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse_str;

use crate::cpp_include::CppInclude;
use crate::type_registry::type_traits::{TypeName, TypesEnum};
use crate::type_registry::{QtType, QtTypeSpanned};
use crate::type_tokens::TypeTokens;

pub fn type_tokens_to_cpp_includes(types: &TypeTokens) -> syn::Result<BTreeSet<CppInclude>> {
    let mut result = BTreeSet::new();

    for ty in types.iter_standard() {
        let ty_info = ty.dyn_type_info();
        if let Some(include) = ty_info.cpp_include() {
            result.insert(CppInclude::new_from_str(&include)?);
        }
    }

    for ty in types.iter_cxx() {
        let ty_info = ty.dyn_type_info();
        if let Some(include) = ty_info.cpp_include() {
            result.insert(CppInclude::new_from_str(&include)?);
        }
    }

    for qt_type in types.iter_qt() {
        let ty = qt_type.get_type();
        let bridge_ty: QtType = match ty {
            QtType::GenericWithArgs(gen_w_args) =>
                gen_w_args.get_monomorphed_type()
                    .ok_or_else(|| syn::Error::new(qt_type.span(), format!("Failed to get monomorphed form of type '{}'", ty.name())))?
                    .into(),
            _ => ty.clone(),
        };

        let ty_info = bridge_ty.dyn_type_info();
        if let Some(include) = ty_info.cpp_include() {
            result.insert(CppInclude::new_from_str(&include)?);
        }
    }

    Ok(result)
}

type BridgeInclude = CppInclude;

/// Definition of type import inside CXX bridge.
/// E.g.
/// ```ignore
/// #[cxx::bridge]
/// pub mod ffi {
///     unsafe extern "C++" {
///     //  =========================================
///         include!("qtbridge-type-lib/src/generated/core/qmetatypeinterface/cpp/qmetatypeinterface.h");
///         #[namespace = "QtPrivate"]
///         type QMetaTypeInterface = super::QMetaTypeInterface;
///     //  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///     }
/// }
/// ```
pub struct TypeImportInBridge {
    include: Option<BridgeInclude>, // The value inside parenthesis of macro 'include!(XYZ)' including quotes or angle brackets delimiters
    namespace: String,      // C++ namespace
    type_lhs: syn::Ident,   // Left side type of the type declaration
    type_rhs: syn::Path,    // Right side type of the type declaration
}

impl TypeImportInBridge {
    pub fn new(include_path: &str, namespace: &str, type_lhs: syn::Ident, type_rhs: syn::Path) -> syn::Result<Self> {

        let include = if include_path.is_empty() { None } else { Some(BridgeInclude::new_from_str(include_path)?) };
        Ok(Self {
            include,
            namespace: namespace.to_owned(),
            type_lhs,
            type_rhs,
        })
    }

    pub fn type_name(&self) -> &syn::Ident {
        &self.type_lhs
    }
}

impl ToTokens for TypeImportInBridge {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut include_macro = None;
        if let Some(include) = &self.include {
            let include_str = format!("include!({})", include.path_with_delims());
            include_macro = Some(parse_str::<syn::Macro>(&include_str)
                .expect("Failed to parse include macro"));
        }

        let mut namespace_attr = None;
        let namespace_str = &self.namespace;
        if !namespace_str.is_empty() {
            namespace_attr = Some(quote! {
                #[namespace = #namespace_str]
            })
        }

        let lhs = &self.type_lhs;
        let rhs = &self.type_rhs;

        quote! {
            #include_macro;
            #namespace_attr
            type #lhs = #rhs;
        }.to_tokens(tokens);
    }
}

pub fn qt_types_to_bridge_imports<'a>(types: impl Iterator<Item = &'a QtTypeSpanned>, is_local_import: bool) -> syn::Result<Vec<TypeImportInBridge>> {

    // Use map for sorting result
    let mut result_map = BTreeMap::new();
    for mut ty in types.cloned() {
        match ty.get_type() {
            QtType::GenericWithArgs(generic_w_args) => {
                if let Some(mono) = generic_w_args.get_monomorphed_type() {
                    // Add monomorphed form of generic type because
                    // it seems that CXX does not work with user defined generic types
                    ty.set_type(mono.into());
                }
            }
            QtType::AliasToMonomorphed(alias) => {
                let mono = alias.get_monomorphed_type()
                    .ok_or_else(|| syn::Error::new(ty.span(), format!("Failed to get monomorphed alias to '{}'", alias.name())))?;
                ty.set_type(mono.into());
            },
            _ => {}
        }

        let ty_info = ty.dyn_type_info();
        let mut ty_comp = ty_info.qualified_path_components();
        let Some(first_comp) = ty_comp.first_mut() else {
            return Err(syn::Error::new(ty.span(), "Qualified path has no components"));
        };

        if is_local_import && *first_comp == "qtbridge_type_lib" {
            *first_comp = "crate";
        }

        let ident = format_ident!("{}", ty_info.name());

        let path_str = ty_comp.join("::");
        if result_map.contains_key(&path_str) {
            continue;
        }
        let path = syn::parse_str(&path_str)?;

        let include = ty_info.cpp_include()
            .unwrap_or_default();
        let namespace = ty_info.cpp_namespace()
            .unwrap_or_default();

        let import = TypeImportInBridge::new(&include, namespace, ident, path)?;
        result_map.insert(path_str, import);
    }

    Ok(result_map
        .into_values()
        .collect())
}

pub fn qt_types_to_rust_import_paths<'a>(types: impl Iterator<Item = &'a QtTypeSpanned>) -> syn::Result<TokenStream> {
    let mut path_map = BTreeMap::<String, Vec<String>>::new();
    for ty in types {
        let ty_info = ty.dyn_type_info();

        let path_before_name = ty_info.path_before_name()
            .unwrap_or_default()
            .to_owned();
        let name = ty_info.name().to_owned();

        path_map.entry(path_before_name)
            .or_default()
            .push(name);
    }

    let mut result = TokenStream::new();
    for (mut path_str, mut type_vec) in path_map {
        if !path_str.is_empty() {
            path_str.push_str("::");
        }

        type_vec.sort_unstable();
        type_vec.dedup();
        let mut types_str = type_vec.join(",");
        if type_vec.len() > 1 {
            types_str = format!("{{{types_str}}}")
        }
        let use_str = format!("use {path_str}{types_str};");
        let use_tokens: TokenStream = syn::parse_str(&use_str)?;
        use_tokens.to_tokens(&mut result);
    }

    Ok(result)
}

