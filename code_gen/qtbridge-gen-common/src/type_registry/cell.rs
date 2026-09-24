// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::type_registry::type_traits::{GenericArgs, StaticTypeGroup, TypeInfo, TypeName};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct CellType {
    rust_name: &'static str,
    arg: Option<Box<syn::Type>>,
}

impl CellType {
    const fn new(rust_name: &'static str) -> Self {
        Self {
            rust_name,
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

impl TypeName for CellType {
    fn name(&self) -> &str {
        self.rust_name
    }

    fn path_before_name(&self) -> Option<&str> {
        Some("std::cell")
    }
}

impl GenericArgs for CellType {
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

impl TypeInfo for CellType {}

impl StaticTypeGroup for CellType {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: [CellType; 1] = [
            CellType::new("RefCell")
        ];

        LIST.as_slice()
    }
}

unsafe impl Send for CellType {}
unsafe impl Sync for CellType {}
