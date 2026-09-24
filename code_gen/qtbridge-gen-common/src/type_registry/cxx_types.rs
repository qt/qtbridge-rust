// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::type_registry;
use type_registry::type_traits::{GenericArgs, StaticTypeGroup, TypeInfo, TypeName};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct CxxType {
    rust_name: &'static str,
    path_before_name: Option<&'static str>,
    cpp_name: Option<&'static str>,
    include: Option<&'static str>,
    arg: Option<Box<syn::Type>>,
}

impl CxxType {
    const fn new(rust_name: &'static str,
        path_before_name: Option<&'static str>,
        cpp_name: Option<&'static str>,
        include: Option<&'static str>) -> Self {
            CxxType {
                rust_name,
                path_before_name,
                cpp_name,
                include,
                arg: None,
            }
        }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self
    }

    pub fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        self
    }
}

impl TypeName for CxxType {
    fn name(&self) -> &str {
        self.rust_name
    }

    fn path_before_name(&self) -> Option<&str> {
        self.path_before_name
    }
}

impl GenericArgs for CxxType {
    fn generic_arg_count(&self) -> usize {
        1
    }

    fn generic_arg_syn(&self, idx: usize) -> Option<syn::Type> {
        assert!(idx == 0);
        self.arg.as_deref()
            .cloned()
    }

    fn set_generic_arg(&mut self, idx: usize, arg: &syn::Type) -> Result<(), String> {
        assert!(idx == 0);
        self.arg = Some(Box::new(arg.clone()));
        Ok(())
    }
}

impl TypeInfo for CxxType {
    fn cpp_name(&self) -> Option<&'static str> {
        self.cpp_name
    }

    fn cpp_include(&self) -> Option<String> {
        self.include.map(String::from)
    }
}

impl StaticTypeGroup for CxxType {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: [CxxType; 2] = [
            // Not really a type from CXX but type that CXX uses a lot.
            // TODO: find a better place for Pin type.
            CxxType::new("Pin", Some("std::pin"), None, None),

            CxxType::new("UniquePtr", Some("cxx"), Some("std::unique_ptr"), Some("<memory>")),
        ];

        &LIST
    }
}

unsafe impl Send for CxxType {}
unsafe impl Sync for CxxType {}
