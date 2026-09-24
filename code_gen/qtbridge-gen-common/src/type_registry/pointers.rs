// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::type_registry::type_traits::{GenericArgs, StaticTypeGroup, TypeInfo, TypeName};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct PointerType {
    rust_name: &'static str,
    path_before_name: Option<&'static str>,
    cpp_name: Option<&'static str>,
    include: Option<&'static str>,
    arg: Option<Box<syn::Type>>
}

impl PointerType {
    const fn new(rust_name: &'static str, path_before_name: Option<&'static str>, cpp_name: Option<&'static str>, include: Option<&'static str>) -> Self {
        Self {
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

impl TypeName for PointerType {
    fn name(&self) -> &str {
        self.rust_name
    }

    fn path_before_name(&self) -> Option<&str> {
        self.path_before_name
    }
}

impl GenericArgs for PointerType {
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

impl TypeInfo for PointerType {

    fn cpp_name(&self) -> Option<&str> {
        self.cpp_name
    }

    fn cpp_include(&self) -> Option<String> {
        self.include.map(String::from)
    }
}

impl StaticTypeGroup for PointerType {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: [PointerType; 2] = [
            PointerType::new("Box", None, Some("rust::Box"), Some(r#""rust/cxx.h""#)),
            PointerType::new("Rc", Some("std::rc"), None, None),
        ];

        LIST.as_slice()
    }
}

unsafe impl Send for PointerType {}
unsafe impl Sync for PointerType {}
