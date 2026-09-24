// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::type_registry::type_traits::{GenericArgs, StaticTypeGroup, TypeName, TypeInfo};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct StandardContainer {
    rust_name: &'static str,
    cpp_name: Option<&'static str>,
    include: Option<&'static str>,
    arg: Option<Box<syn::Type>>,
}

impl StandardContainer {
    const fn new(rust_name: &'static str, cpp_name: Option<&'static str>, include: Option<&'static str>) -> Self {
        Self {
            rust_name,
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

impl TypeName for StandardContainer {
    fn name(&self) -> &str {
        self.rust_name
    }

    fn path_before_name(&self) -> Option<&str> {
        None
    }
}

impl GenericArgs for StandardContainer {
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

impl TypeInfo for StandardContainer {
    fn cpp_name(&self) -> Option<&'static str> {
        self.cpp_name
    }

    fn cpp_include(&self) -> Option<String> {
        self.include.map(String::from)
    }
}

impl StaticTypeGroup for StandardContainer {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: [StandardContainer; 1] = [
            StandardContainer::new("Vec", Some("rust::Vec"), Some(r#""rust/cxx.h""#)),
        ];

        &LIST
    }
}

unsafe impl Send for StandardContainer {}
unsafe impl Sync for StandardContainer {}
