// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::type_registry::type_traits::{GenericArgs, StaticTypeGroup, TypeInfo, TypeName};

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct ValueHolder {
    rust_name: &'static str,
    args: [Option<Box<syn::Type>>; 2],
    max_arg_count: u8,
}

impl ValueHolder {
    const fn new(rust_name: &'static str, arg_count: u8) -> Self{
        Self {
            rust_name,
            args: [None, None],
            max_arg_count: if arg_count <= 2 { arg_count } else { 2 },
        }
    }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self
    }

    pub fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        self
    }
}

impl TypeName for ValueHolder {
    fn name(&self) -> &str {
        self.rust_name
    }

    fn path_before_name(&self) -> Option<&str> {
        None
    }
}

impl GenericArgs for ValueHolder {
    fn generic_arg_count(&self) -> usize {
        self.max_arg_count as usize
    }

    fn generic_arg_syn(&self, idx: usize) -> Option<syn::Type> {
        if idx >= self.max_arg_count as usize {
            return None
        }
        self.args[idx].as_deref()
            .cloned()
    }

    fn set_generic_arg(&mut self, idx: usize, arg_value: &syn::Type) -> Result<(), String> {
        if idx >= self.max_arg_count as usize {
            return Err("Generic argument index is out of bounds".into())
        }
        self.args[idx] = Some(Box::new(arg_value.clone()));
        Ok(())
    }
}

impl TypeInfo for ValueHolder {}

impl StaticTypeGroup for ValueHolder {
    fn get_static_sorted_list() -> &'static [Self] {
        static LIST: [ValueHolder; 2] = [
            ValueHolder::new("Result", 2),
            ValueHolder::new("Option", 1),
        ];

        &LIST
    }
}

unsafe impl Send for ValueHolder {}
unsafe impl Sync for ValueHolder {}
