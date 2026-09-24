// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::type_registry;
use type_registry::QtType;
use type_registry::TypeCategory;
use type_registry::type_traits::{FindType, GenericArgs, MetaTypeId, TypeName, TypeInfo};

use super::common::get_include_path;
use super::generic::QtGenericTypeWithArgs;

/// Instantiation of generic Qt type for certain type(s)
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct QtMonomorphedType {
    name: String,
    path_in_gen: String,
    source: Box<QtGenericTypeWithArgs>,
    metatypeid: MetaTypeId,
}

impl QtMonomorphedType {
    pub fn new(name: String, path_in_gen: String, source: QtGenericTypeWithArgs, metatypeid: MetaTypeId) -> Self {
        Self {
            name,
            path_in_gen,
            source: Box::new(source),
            metatypeid
        }
    }

    // The overload of new() that accepts arguments as '&str' instead of 'String'
    // to avoid conversions on the caller side and make the code shorter.
    pub fn new_str(name: &str, path_in_gen: &str, source: QtGenericTypeWithArgs, metatypeid: MetaTypeId) -> Self {
        Self::new(name.into(), path_in_gen.into(), source, metatypeid)
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

    pub fn source(&self) -> &QtGenericTypeWithArgs {
        self.source.as_ref()
    }
}

impl TypeName for QtMonomorphedType {
    fn name(&self) -> &str {
        &self.name
    }

    fn path_before_name(&self) -> Option<&str> {
        Some("qtbridge_type_lib")
    }
}

impl GenericArgs for QtMonomorphedType {}

impl TypeInfo for QtMonomorphedType {
    fn cpp_name(&self) -> Option<&str> {
        Some(self.name.as_str())
    }

    fn cpp_include(&self) -> Option<String> {
        let include = get_include_path(&self.path_in_gen, self.name())
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

impl FindType for QtMonomorphedType {
    fn find_by_name(name: &str) -> Option<Self> {
        let qt = QtType::find_by_name(name)?;
        match qt {
            QtType::GenericMonomorphed(mono) => Some(mono),
            _ => None,
        }
    }
}
