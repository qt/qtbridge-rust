// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::type_registry;
use crate::type_registry::type_traits::GenericArgs;
use type_registry::QtType;
use type_registry::type_traits::{FindType, MetaTypeId, TypeCategory, TypeName, TypeInfo};
use type_registry::qt::common::get_include_path;
use type_registry::qt::monomorphed::QtMonomorphedType;

/// Alias to monomorphed generic type
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct QtAliasToMonomorphedType {
    name: String,
    alias_to: String,
    path_in_gen: String,
    metatypeid: MetaTypeId,
}

impl QtAliasToMonomorphedType {
    pub fn new(name: String, alias_to: String, path_in_gen: String, metatypeid: MetaTypeId) -> Self {
        Self {
            name, alias_to, path_in_gen, metatypeid
        }
    }

    // The overload of new() that accepts arguments as '&str' instead of 'String'
    // to avoid conversions on the caller side and make the code shorter.
    pub fn new_str(name: &str, alias_to: &str, path_in_gen: &str, metatypeid: MetaTypeId) -> Self {
        Self::new(name.into(), alias_to.into(), path_in_gen.into(), metatypeid)
    }

    pub fn path_in_gen(&self) -> &str {
        &self.path_in_gen
    }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self
    }

    pub fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        self
    }

    pub fn to(&self) -> &str {
        &self.alias_to
    }

    pub fn get_monomorphed_type(&self) -> Option<QtMonomorphedType> {
        QtMonomorphedType::find_by_name(self.to())
    }
}

impl TypeName for QtAliasToMonomorphedType {
    fn name(&self) -> &str {
        &self.name
    }

    fn path_before_name(&self) -> Option<&str> {
        Some("qtbridge_type_lib")
    }
}

impl GenericArgs for QtAliasToMonomorphedType {}

impl TypeInfo for QtAliasToMonomorphedType {
    fn cpp_name(&self) -> Option<&str> {
        Some(self.name.as_str())
    }

    fn cpp_include(&self) -> Option<String> {
        let include = get_include_path(&self.path_in_gen, &self.alias_to)
            .expect("Failed to get include path");
        Some(format!("\"{include}\""))
    }

    fn metatype_id(&self) -> MetaTypeId {
        self.metatypeid.clone()
    }

    fn category(&self) -> TypeCategory {
        TypeCategory::Qt
    }
}

impl FindType for QtAliasToMonomorphedType {
    fn find_by_name(name: &str) -> Option<Self> {
        let qt = QtType::find_by_name(name)?;
        match qt {
            QtType::AliasToMonomorphed(alias) => Some(alias),
            _ => None,
        }
    }
}
