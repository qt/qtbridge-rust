// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::type_registry;
use type_registry::QtType;
use type_registry::TypeCategory;
use type_registry::type_traits::{FindType, GenericArgs, MetaTypeId, TypeInfo, TypeName};
use super::common::get_include_path;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct QtNonGenericType {
    name: String,
    path_in_gen: String,
    metatypeid: MetaTypeId,
    namespace: String,
}

impl QtNonGenericType {
    pub fn new(name: String, path_in_gen: String, metatypeid: MetaTypeId, namespace: String) -> Self{
        Self {
            name, path_in_gen, metatypeid, namespace
        }
    }

    // The overload of new() that accepts arguments as '&str' instead of 'String'
    // to avoid conversions on the caller side and make the code shorter.
    pub fn new_str(name: &str, path_in_gen: &str, metatypeid: MetaTypeId, namespace: &str) -> Self{
        Self::new(name.into(), path_in_gen.into(), metatypeid, namespace.into())
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
}

impl TypeName for QtNonGenericType {
    fn name(&self) -> &str {
        &self.name
    }

    fn path_before_name(&self) -> Option<&str> {
        Some("qtbridge_type_lib")
    }
}

impl GenericArgs for QtNonGenericType {}

impl TypeInfo for QtNonGenericType {
    fn cpp_name(&self) -> Option<&str> {
        Some(self.name.as_str())
    }

    fn cpp_include(&self) -> Option<String> {
        let include = get_include_path(&self.path_in_gen, self.name())
            .expect("Failed to get include path");
        Some(format!("\"{include}\""))
    }

    fn cpp_namespace(&self) -> Option<&str> {
        let ns = &self.namespace;
        (!ns.is_empty()).then_some(ns.as_str())
    }

    fn metatype_id(&self) -> MetaTypeId {
        self.metatypeid.clone()
    }

    fn category(&self) -> TypeCategory {
        TypeCategory::Qt
    }
}

impl FindType for QtNonGenericType {
    fn find_by_name(name: &str) -> Option<Self> {
        let qt = QtType::find_by_name(name)?;
        match qt {
            QtType::NonGeneric(non_generic) => Some(non_generic),
            _ => None,
        }
    }
}
