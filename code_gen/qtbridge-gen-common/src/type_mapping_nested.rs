// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::VecDeque;

use syn::punctuated::Punctuated;
use syn::visit_mut::{VisitMut, visit_type_mut};

use crate::type_mapping::TypeMapping;
use crate::type_to_string::{path_to_string_fallback, type_to_string_fallback};


// TypeMapping that works with more high level (potentially recursive) AST entities.
pub struct TypeMappingNested<T: TypeMapping> {
    /// Object implementing mapping
    map_impl: T,
}

impl<T: TypeMapping> TypeMappingNested<T> {
    pub fn new(map_impl: T) -> Self {
        Self {
            map_impl,
        }
    }

    pub fn get_impl(&self) -> &T {
        &self.map_impl
    }

    pub fn map_impl_item(&self, src: &syn::ImplItem) -> syn::Result<syn::ImplItem> {
        let mut result = src.clone();
        let mut v = Visitor::new(self.get_impl());
        v.visit_impl_item_mut(&mut result);
        v.result().map(|_| result)
    }

    pub fn map_item_fn(&self, src: &syn::ItemFn) -> syn::Result<syn::ItemFn> {
        let mut result = src.clone();
        let mut v = Visitor::new(self.get_impl());
        v.visit_item_fn_mut(&mut result);
        v.result().map(|_| result)
    }

    pub fn map_signature(&self, src: &syn::Signature) -> syn::Result<syn::Signature> {
        let mut result = src.clone();
        let mut v = Visitor::new(self.get_impl());
        v.visit_signature_mut(&mut result);
        v.result().map(|_| result)
    }

    pub fn map_type(&self, src: &syn::Type) -> syn::Result<syn::Type> {
        let mut result = src.clone();
        let mut v = Visitor::new(self.get_impl());
        v.visit_type_mut(&mut result);
        v.result().map(|_| result)
    }

    pub fn map_path(&self, src: &syn::Path) -> syn::Result<syn::Path> {
        let mut result = src.clone();
        let mut v = Visitor::new(self.get_impl());
        v.visit_path_mut(&mut result);
        v.result().map(|_| result)
    }
}

struct Visitor<'a, T: TypeMapping> {
    map: &'a T,
    result: VecDeque<syn::Result<()>>,
}

impl<'a, T: TypeMapping> Visitor<'a, T> {
    pub fn new(map: &'a T) -> Self {
        Self {
            map,
            result: VecDeque::new()
        }
    }

    pub fn result(&mut self) -> syn::Result<()> {
        self.result.pop_front().unwrap_or(Ok(()))
    }
}

impl<'a, T: TypeMapping> VisitMut for Visitor<'a, T> {
    fn visit_type_mut(&mut self, src: &mut syn::Type) {
        let syn::Type::Path(type_path) = src else {
            visit_type_mut(self, src);
            return;
        };

        // The type represents a single ident.
        let segments = &type_path.path.segments;
        if segments.len() == 1 &&
            let Some(seg) = segments.first() &&
            matches!(seg.arguments, syn::PathArguments::None)
        {
            if let Some(mapped_type) = self.map.map(&seg.ident) {
                *src = mapped_type;
            }
            return;
        }

        visit_type_mut(self, src);
    }

    fn visit_path_mut(&mut self, src: &mut syn::Path) {
        // There are multiple segments in the type path.
        let mut new_segs = Punctuated::new();

        for seg in &src.segments {
            match seg.arguments {
                // We map either segment ident or segment argument but not both
                syn::PathArguments::None => {
                    let seg_ident = &seg.ident;
                    let mapped = self.map.map(seg_ident);
                    match mapped {
                        Some(mapped_type) => {
                            if let syn::Type::Path(mapped_type_path) = &mapped_type &&
                                mapped_type_path.qself.is_none()
                            {
                                new_segs.extend(mapped_type_path.path.segments.clone());
                            }
                            else {
                                self.result.push_back(
                                    Err(syn::Error::new(
                                        seg_ident.span(),
                                        format!("Failed to map '{seg_ident}' to '{}'. Non-Path type can't be substituted in path '{}'", type_to_string_fallback(&mapped_type), path_to_string_fallback(&src)))));
                                new_segs.push(seg.clone());
                            }
                        }
                        None => new_segs.push(seg.clone()),
                    }
                },
                _ => {
                    let mut new_seg = seg.clone();
                    self.visit_path_segment_mut(&mut new_seg);
                    new_segs.push(new_seg);
                }
            }
        }

        src.segments = new_segs;
    }
}
