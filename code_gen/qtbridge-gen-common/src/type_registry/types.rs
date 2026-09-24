// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::type_traits::{FindType, TypeCategory, TypesEnum, TypeInfo};
use super::cxx_types::CxxType;
use super::standards::StandardType;
use super::qt::QtType;

#[derive(Clone)]
pub enum Type {
    Standard(StandardType),
    Cxx(CxxType),
    Qt(QtType),
}

impl Type {
    pub fn find_by_name_in_category(name: &str, category: &TypeCategory) -> Option<Self> {
        match category {
            TypeCategory::Standard => StandardType::find_by_name(name)
                .map(Type::from),
            TypeCategory::Cxx => CxxType::find_by_name(name)
                .map(Type::from),
            TypeCategory::Qt => QtType::find_by_name(name)
                .map(Type::from),
        }
    }

    pub fn find_by_ident_in_category_result(ident: &syn::Ident, category: &TypeCategory) -> syn::Result<Self> {
        Self::find_by_name_in_category(&ident.to_string(), category)
            .ok_or_else(|| syn::Error::new(ident.span(), format!("Failed to find type by ident '{ident}' in category {category}")))
    }

    pub fn find_by_ident_result(ident: &syn::Ident) -> syn::Result<Self> {
        Self::find_by_name(&ident.to_string())
            .ok_or_else(|| syn::Error::new(ident.span(), format!("Failed to find type by ident '{ident}'")))
    }

    pub fn find_by_ident_in_opt_category_result(ident: &syn::Ident, category: Option<&TypeCategory>) -> syn::Result<Self> {
        match category {
            Some(category) => Self::find_by_ident_in_category_result(ident, category),
            None => Self::find_by_ident_result(ident),
        }
    }
}

impl TypesEnum for Type {
    fn dyn_type_info(&self) -> &dyn TypeInfo {
        match self {
            Type::Standard(standard) => standard.dyn_type_info(),
            Type::Cxx(cxx) => cxx.dyn_type_info(),
            Type::Qt(qt) => qt.dyn_type_info(),
        }
    }

    fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        match self {
            Type::Standard(standard) => standard.mut_dyn_type_info(),
            Type::Cxx(cxx) => cxx.mut_dyn_type_info(),
            Type::Qt(qt) => qt.mut_dyn_type_info(),
        }
    }
}

impl FindType for Type {
    fn find_by_name(name: &str) -> Option<Self> {
        StandardType::find_by_name(name).map(Type::from)
            .or_else(|| CxxType::find_by_name(name).map(Type::from))
            .or_else(|| QtType::find_by_name(name).map(Type::from))
    }

    fn find_by_path(path: &syn::Path) -> Option<Self> {
        StandardType::find_by_path(path).map(Type::from)
            .or_else(|| CxxType::find_by_path(path).map(Type::from))
            .or_else(|| QtType::find_by_path(path).map(Type::from))
    }
}

impl From<StandardType> for Type {
    fn from(value: StandardType) -> Self {
        Type::Standard(value)
    }
}

impl From<CxxType> for Type {
    fn from(value: CxxType) -> Self {
        Type::Cxx(value)
    }
}

impl From<QtType> for Type {
    fn from(value: QtType) -> Self {
        Type::Qt(value)
    }
}
