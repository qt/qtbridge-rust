// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::ToTokens;
use syn::spanned::Spanned;

use crate::type_registry;
use type_registry::type_traits::{FindType, GenericArgs, TypeName, TypeInfo};
use type_registry::{PrimitiveType, StandardType, TypeCategory};
use crate::type_utils::{path_from_type, path_to_type};
use crate::type_to_string::{type_to_ident_str, type_to_string_fallback};
use super::monomorphed::QtMonomorphedType;
use super::non_generic::QtNonGenericType;
use super::QtType;

/// Generic Qt type having some generic parameters
/// but concrete types for them are not specified.
/// E.g. List, QHash, QMap
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct QtGenericTypeWithoutArgs {
    /// Name of the generic type (before angle brackets)
    gen_name: String,

    /// The full name of the generic type, including the angle-bracketed type arguments
    /// (but without leading path segments).
    name_with_args: String,

    /// Path to the type relative to 'generated' folder
    path_in_gen: String,

    /// Idents of the generic args as they were defined in input file (e.g.: K, V)
    args: Vec<String>,
}

impl QtGenericTypeWithoutArgs {
    pub fn new(gen_name: String, path_in_gen: String, args: Vec<String>) -> Self {
        let args_str = args.join(",");
        let name_with_args = format!("{gen_name}<{args_str}>");

        Self {
            gen_name,
            name_with_args,
            path_in_gen,
            args,
        }
    }

    // The overload of new() that accepts arguments as '&str' instead of 'String'
    // to avoid conversions on the caller side and make the code shorter.
    pub fn new_str(gen_name: &str, path_in_gen: &str, args: &[&str]) -> Self {
        Self::new(
            gen_name.into(),
            path_in_gen.into(),
            args.iter()
                .map(ToString::to_string)
                .collect())
    }

    /// Return the ident string of the generic type (before angle brackets)
    pub fn gen_name(&self) -> &str {
        &self.gen_name
    }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self
    }

    pub fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        self
    }

    pub fn path_in_gen(&self) -> &str {
        &self.path_in_gen
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }

    pub fn set_args(&self, args: Vec<QtGenericArg>) -> Result<QtGenericTypeWithArgs, String> {
        if self.args.len() != args.len() {
            return Err(format!("Mismatch in number of generic arguments of Qt type: got {}, expected {}", args.len(), self.args.len()))
        }

        Ok(QtGenericTypeWithArgs {
            gen_name: self.gen_name().into(),
            name_with_args: QtGenericTypeWithArgs::get_name_with_args(self.gen_name(), args.iter()),
            path_in_gen: self.path_in_gen().into(),
            args,
        })
    }

    pub fn set_args_from_syn_generic_args(&self, src: &syn::AngleBracketedGenericArguments) -> syn::Result<QtGenericTypeWithArgs> {
        let args = src.args.iter()
            .map(QtGenericArg::try_from)
            .collect::<syn::Result<Vec<_>>>()?;
        if self.args.len() != args.len() {
            return Err(syn::Error::new(src.span(), format!("Mismatch in generic argument count: expected {} vs found {}", self.args.len(), args.len())))
        }

        self.set_args(args)
            .map_err(|err| syn::Error::new(src.span(), err))
    }

}

impl std::fmt::Display for QtGenericTypeWithoutArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.full_name())
    }
}

impl TypeName for QtGenericTypeWithoutArgs {
    fn name(&self) -> &str {
        self.gen_name()
    }

    fn full_name(&self) -> &str {
        &self.name_with_args
    }

    fn path_before_name(&self) -> Option<&str> {
        Some("qtbridge_type_lib")
    }
}

impl GenericArgs for QtGenericTypeWithoutArgs {
    fn generic_arg_count(&self) -> usize {
        self.args.len()
    }

    fn generic_arg_syn(&self, idx: usize) -> Option<syn::Type> {
        let arg = self.args.get(idx)?;
        Some(syn::parse_str(arg).unwrap())
    }
}

impl TypeInfo for QtGenericTypeWithoutArgs {
    fn cpp_name(&self) -> Option<&str> {
        Some(self.gen_name())
    }

    fn cpp_include(&self) -> Option<String> {
        Some(format!("<{}>", self.name()))
    }

    fn category(&self) -> TypeCategory {
        TypeCategory::Qt
    }
}

impl FindType for QtGenericTypeWithoutArgs {
    fn find_by_name(mut name: &str) -> Option<Self> {
        if let Some(bracket_pos) = name.find('<') {
            name = &name[..bracket_pos];
        }
        let qt = QtType::find_by_name(name)?;
        match qt {
            QtType::GenericWithoutArgs(generic) => Some(generic),
            _ => None,
        }
    }
}


/// Generic Qt type having some types
/// assigned to generic parameters
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct QtGenericTypeWithArgs {
    /// Name of the generic type (before angle brackets)
    gen_name: String,

    /// Whole name of the generic type including types in angle brackets
    name_with_args: String,

    /// Path to the type relative to 'generated' folder
    path_in_gen: String,

    /// Generic args
    args: Vec<QtGenericArg>,
}

impl QtGenericTypeWithArgs {
    pub fn new(gen_name: String, path_in_gen: String, args: Vec<QtGenericArg>) -> Self {
        let name_with_args = Self::get_name_with_args(&gen_name, args.iter());
        Self {
            gen_name,
            name_with_args,
            path_in_gen,
            args,
        }
    }

    pub fn get_name_with_args<'a>(gen_name: &str, args: impl Iterator<Item = &'a (impl ToString + 'a)>) -> String {
        let args_str = args
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");
        format!("{gen_name}<{args_str}>")
    }

    pub fn gen_name(&self) -> &str {
        &self.gen_name
    }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self
    }

    pub fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        self
    }

    pub fn path_in_gen(&self) -> &str {
        &self.path_in_gen
    }

    pub fn args(&self) -> &[QtGenericArg] {
        &self.args
    }

    /// Return the name of concrete type obtained after generic arguments substitution
    pub fn get_monomorphed_type_name(&self) -> Option<String> {
        let mut result = self.gen_name().to_owned();

        for arg in &self.args {
            let arg_type_str = match arg {
                QtGenericArg::Primitive(primitive) => primitive.name().to_owned(),
                QtGenericArg::Qt(qt_concrete) => qt_concrete.name().to_owned(),
                QtGenericArg::Unclassified(unclassified) => type_to_ident_str(unclassified)
                    .ok()?
            };
            result.push('_');
            result.push_str(&arg_type_str);
        }

        Some(result)
    }

    pub fn get_monomorphed_type(&self) -> Option<QtMonomorphedType> {
        QtMonomorphedType::find_by_name(
            &self.get_monomorphed_type_name()?)
    }

    pub fn get_generic_type(&self) -> QtGenericTypeWithoutArgs {
        QtGenericTypeWithoutArgs::find_by_name(self.name())
            .unwrap_or_else(|| {
                panic!("Failed to find generic without args form of '{}'", self)
            })
    }
}

impl std::fmt::Display for QtGenericTypeWithArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.full_name())
    }
}

impl TypeName for QtGenericTypeWithArgs {
    fn name(&self) -> &str {
        self.gen_name()
    }

    fn full_name(&self) -> &str {
        &self.name_with_args
    }

    fn path_before_name(&self) -> Option<&str> {
        Some("qtbridge_type_lib")
    }
}

impl GenericArgs for QtGenericTypeWithArgs {
    fn generic_arg_count(&self) -> usize {
        self.args.len()
    }

    fn generic_arg_syn(&self, idx: usize) -> Option<syn::Type> {
        let arg = self.args.get(idx)?;
        match arg {
            QtGenericArg::Primitive(primitive) =>
                syn::parse_str(&primitive.qualified_path_string())
                    .ok(),
            QtGenericArg::Qt(qt_ty) =>
                syn::parse_str(&qt_ty.qualified_path_string())
                    .ok(),
            QtGenericArg::Unclassified(ty) =>
                Some(ty.clone()),
        }
    }
}

impl TypeInfo for QtGenericTypeWithArgs {
    fn cpp_name(&self) -> Option<&str> {
        Some(self.gen_name())
    }

    fn cpp_include(&self) -> Option<String> {
        Some(format!("<{}>", self.name()))
    }

    fn category(&self) -> TypeCategory {
        TypeCategory::Qt
    }
}


#[derive(Clone, Eq, Hash, PartialEq)]
pub enum QtGenericArg {
    Primitive(PrimitiveType),
    Qt(QtNonGenericType),
    Unclassified(syn::Type),
}

impl std::fmt::Display for QtGenericArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(primitive) => f.write_str(primitive.name()),
            Self::Qt(qt) => f.write_str(qt.name()),
            Self::Unclassified(path) => f.write_str(&type_to_string_fallback(path)),
        }
    }
}

impl From<PrimitiveType> for QtGenericArg {
    fn from(value: PrimitiveType) -> Self {
        Self::Primitive(value)
    }
}

impl From<QtNonGenericType> for QtGenericArg {
    fn from(value: QtNonGenericType) -> Self {
        Self::Qt(value)
    }
}

impl TryFrom<&type_registry::Type> for QtGenericArg {
    type Error = ();

    fn try_from(value: &type_registry::Type) -> Result<Self, ()> {
        match value {
            type_registry::Type::Standard(StandardType::Primitive(primitive)) => {
                return Ok((*primitive).clone().into())
            },
            type_registry::Type::Qt(QtType::NonGeneric(qt_non_generic)) => {
                return Ok(qt_non_generic.clone().into())
            },
            _ => {},
        }
        Err(())
    }
}

impl TryFrom<&syn::GenericArgument> for QtGenericArg {
    type Error = syn::Error;

    fn try_from(value: &syn::GenericArgument) -> Result<Self, Self::Error> {
        let syn::GenericArgument::Type(arg_type) = value else {
            return Err(syn::Error::new(value.span(), format!("Unsupported category '{:?}' of GenericArgument '{}'", std::mem::discriminant(value), value.to_token_stream())))
        };

        Self::try_from(arg_type)
    }
}

impl TryFrom<&syn::Type> for QtGenericArg {
    type Error = syn::Error;
    fn try_from(value: &syn::Type) -> syn::Result<Self> {
        path_from_type(value)
            .map_or_else(|_| Ok(Self::Unclassified(value.clone())), |path| Self::try_from(path))
    }
}

impl TryFrom<&syn::Path> for QtGenericArg {
    type Error = syn::Error;

    fn try_from(path: &syn::Path) -> syn::Result<Self> {
        if let Some(primitive) = PrimitiveType::find_by_path(path) {
            return Ok(Self::Primitive(primitive))
        }
        if let Some(qt) = QtType::find_by_path(path) {
            match qt {
                QtType::NonGeneric(concrete) => return Ok(Self::Qt(concrete)),
                _ => return Err(syn::Error::new(path.span(), "Qt types other than non generic are not supported currently as elements of another generic type")),
            }
        }
        if let Some(ty) = type_registry::Type::find_by_path(path) {
            return Err(syn::Error::new(path.span(), format!("Type '{}' is not supported as elements of Qt generic type", ty.qualified_path_string())))
        }

        Ok(Self::Unclassified(path_to_type(path.clone())))
    }
}

impl TryFrom<&str> for QtGenericArg {
    type Error = syn::Error;

    fn try_from(src: &str) -> syn::Result<Self> {
        let ty: syn::Type = syn::parse_str(src)?;
        Self::try_from(&ty)
    }
}

unsafe impl Sync for QtGenericArg {}
unsafe impl Send for QtGenericArg {}
