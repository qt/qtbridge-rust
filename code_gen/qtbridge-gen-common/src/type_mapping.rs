// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::type_mapping_nested::TypeMappingNested;

/// Trait intended to substitute a type given as Ident
/// for another type specified as syn::Type
/// in different syntax constructions.
pub trait TypeMapping {
    fn map(&self, key: &syn::Ident) -> Option<syn::Type>;

    fn new_nested(self) -> TypeMappingNested<Self> where Self: Sized {
        TypeMappingNested::new(self)
    }
}
