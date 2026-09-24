// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::type_registry::type_traits::{GenericArgs, StaticTypeGroup, TypeInfo, TypeName};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct StringType {
    rust_name: &'static str,
    cpp_name: &'static str,
}

impl StringType {
    const fn new(rust_name: &'static str, cpp_name: &'static str) -> StringType {
        Self {rust_name, cpp_name }
    }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self
    }

    pub fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        self
    }
}

impl TypeName for StringType {
    fn name(&self) -> &str {
        self.rust_name
    }

    fn path_before_name(&self) -> Option<&str> {
        None
    }
}

impl GenericArgs for StringType {}

impl TypeInfo for StringType {
    fn cpp_name(&self) -> Option<&'static str> {
        Some(self.cpp_name)
    }

    fn cpp_include(&self) -> Option<String> {
        Some(r#""rust/cxx.h""#.into())
    }
}

impl StaticTypeGroup for StringType {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: [StringType; 2] = [
            StringType::new("String", "rust::String"),
            StringType::new("str", "rust::Str"),
        ];
        &LIST
    }
}
