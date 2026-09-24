// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::format_ident;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;

use crate::type_registry::type_traits::{FindType, TypeName};
use crate::type_registry::qt::monomorphed_alias::QtAliasToMonomorphedType;

pub struct QtAliasMapping {
    last_error: syn::Result<()>,
}

impl Default for QtAliasMapping {
    fn default() -> Self {
        Self::new()
    }
}

impl QtAliasMapping {
    pub fn new() -> Self {
        Self {
            last_error: Ok(())
        }
    }

    pub fn result(&self) -> syn::Result<()> {
        self.last_error.clone()
    }

    fn do_visit_path(&mut self, src: &mut syn::Path) -> syn::Result<()> {
        if let Some(qt_alias) = QtAliasToMonomorphedType::find_by_path(src) {
            let pos = src.segments.iter()
                .position(|seg| seg.ident == qt_alias.name())
                .ok_or_else(|| syn::Error::new(src.span(), "Failed to find segment with Qt alias type"))?;
            src.segments[pos].ident = format_ident!("{}", qt_alias.to());
            src.segments.iter_mut()
                .for_each(|seg| self.visit_path_segment_mut(seg));
        }
        Ok(())
    }
}

impl VisitMut for QtAliasMapping {
    fn visit_path_mut(&mut self, src: &mut syn::Path) {
        if let Err(err) = self.do_visit_path(src) {
            self.last_error = Err(err);
        }
    }
}
