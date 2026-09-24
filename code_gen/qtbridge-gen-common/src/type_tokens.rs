// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::type_registry;
use type_registry::{CxxType, QtType, QtTypeSpanned, StandardType};
use type_registry::qt::generic::QtGenericArg;
use type_registry::type_traits::FindType;
use crate::type_to_string::path_segment_to_string;
use crate::type_utils::{are_all_args_generic_idents, get_ident_of_last_path_segment_or_err, ident_to_path, path_from_type};


/// Struct to collect, categorize and hold types gathered by visiting AST entities
#[derive(Default)]
pub struct TypeTokens {
    generic_idents: Vec<syn::Ident>,
    unclassified: HashSet<syn::Path>,
    standard: HashSet<StandardType>,
    cxx: HashSet<CxxType>,
    qt: HashSet<QtTypeSpanned>
}

impl TypeTokens {
    pub fn new_for_generic(generic_idents: &[syn::Ident]) -> Self {
        Self {
            generic_idents: generic_idents.into(),
            unclassified: HashSet::new(),
            standard: HashSet::new(),
            cxx: HashSet::new(),
            qt: HashSet::new(),
        }
    }

    pub fn generic_idents(&self) -> &[syn::Ident] {
        &self.generic_idents
    }

    pub fn add_generic_ident(&mut self, ident: &syn::Ident) -> syn::Result<()> {
        if self.generic_idents.contains(ident) {
            return Err(syn::Error::new(ident.span(), format!("Generic ident '{ident}' is already in the list")))
        }

        self.generic_idents.push(ident.clone());
        Ok(())
    }

    pub fn remove_generic_ident(&mut self, ident: &syn::Ident) -> syn::Result<()> {
        let pos = self.generic_idents.iter()
            .position(|i| i == ident);

        match pos {
            Some(pos) => {
                self.generic_idents.remove(pos);
                Ok(())
            },
            None => Err(syn::Error::new(ident.span(), format!("Generic ident '{ident}' is not in the list"))),
        }
    }

    pub fn iter_unclassified(&self) -> impl Iterator<Item = &syn::Path> {
        self.unclassified.iter()
    }

    pub fn iter_standard(&self) -> impl Iterator<Item = &StandardType> {
        self.standard.iter()
    }

    pub fn iter_cxx(&self) -> impl Iterator<Item = &CxxType> {
        self.cxx.iter()
    }

    pub fn iter_qt(&self) -> impl Iterator<Item = &QtTypeSpanned> {
        self.qt.iter()
    }

    pub fn contains_unclassified(&self, value: &syn::Path) -> bool {
        self.unclassified.contains(value)
    }

    pub fn collect_from_signature(&mut self, src: &syn::Signature) -> syn::Result<()> {
        let mut v = Visitor::new(self);
        src.inputs.iter()
            .for_each(|arg| {
                v.visit_fn_arg(arg)
            });
        if let syn::ReturnType::Type(_, ty) = &src.output {
            v.visit_type(ty)
        }
        v.result()
    }

    pub fn collect_from_path(&mut self, src: &syn::Path) -> syn::Result<()> {
        let mut v = Visitor::new(self);
        v.visit_path(src);
        v.result()
    }

    pub fn collect_from_type(&mut self, src: &syn::Type) -> syn::Result<()> {
        let mut v = Visitor::new(self);
        v.visit_type(src);
        v.result()
    }

    pub fn insert_ident_type(&mut self, ident: syn::Ident) {
        match type_registry::Type::find_by_name(&ident.to_string()) {
            Some(type_registry::Type::Standard(standard)) =>
                self.standard.insert(standard.clone()),
            Some(type_registry::Type::Cxx(cxx)) =>
                self.cxx.insert(cxx.clone()),
            Some(type_registry::Type::Qt(qt)) =>
                self.qt.insert(QtTypeSpanned::new(qt, ident.span())),
            None =>
                self.unclassified.insert(ident.into())
        };
    }

    fn insert_standard(&mut self, value: StandardType) {
        self.standard.insert(value);
    }

    fn insert_cxx(&mut self, value: CxxType) {
        self.cxx.insert(value);
    }

    fn insert_qt(&mut self, value: QtTypeSpanned) {
        self.qt.insert(value);
    }

    fn insert_unclassified(&mut self, value: syn::Path) {
        self.unclassified.insert(value);
    }

    pub fn remove_unclassified(&mut self, value: &syn::Path) -> bool {
        self.unclassified.remove(value)
    }

    pub fn remove_qt(&mut self, value: &syn::Path) -> bool {
        if let Some(qt_type) = QtType::find_by_path(value) {
            return self.qt.remove(&QtTypeSpanned::new(qt_type, value.span()))
        }
        false
    }

    /// Iterate type tokens previously unclassified and check if they can be classified as Qt types now.
    /// This may happen during Qt type lib generation:
    /// First type is discovered as dependency for some other type (as type of function/trait argument) and seems unknown.
    /// Later, when we processed a definition of this type in dedicated file, we figure out that it actually belongs to Qt types.
    pub fn check_unclassified(&mut self) -> syn::Result<()> {
        let mut error = None;

        let mut unclassified = std::mem::take(&mut self.unclassified);
        unclassified.retain(|path| {
            let Some(qt_type) = QtType::find_by_path(path) else {
                // The type is still unknown. Leave it in the collection.
                return true
            };

            if let QtType::GenericWithArgs(generic_w_args) = &qt_type {
                if let Some(monomorphed) = generic_w_args.get_monomorphed_type() {
                    // Monomorphed version of generic is known - then add it to qt category
                    // and remove from unclassified
                    self.qt.insert(QtTypeSpanned::new(monomorphed.into(), path.span()));
                    return false
                }
                else {
                    // Check if all of arguments are generic type specifier (e.g. QMap<K, V>)
                    let are_all_args_generic_idents = generic_w_args.args().iter()
                        .all(|arg| {
                            let QtGenericArg::Unclassified(ty) = arg else {
                                return false
                            };
                            let Ok(path) = path_from_type(ty) else {
                                return false
                            };
                            path.get_ident()
                                .is_some_and(|ident| self.generic_idents().contains(ident))
                        });
                    if are_all_args_generic_idents {
                        self.qt.insert(QtTypeSpanned::new(generic_w_args.get_generic_type().into(), path.span()));
                        return false
                    }
                }

                // Some of generic arguments are unclassified or monomorphed type is not known yet
                return true
            }

            let Some(last_seg) = path.segments.last() else {
                return false
            };
            let last_seg_str = match path_segment_to_string(last_seg) {
                Ok(str) => str,
                Err(err) => {
                    error = Some(err);
                    return true
                }
            };
            let Some(qt_type) = QtType::find_by_name(&last_seg_str) else {
                return true
            };
            self.qt.insert(QtTypeSpanned::new(qt_type, path.span()));
            false
        });
        self.unclassified = unclassified;

        error.map_or(Ok(()), Err)
    }

    pub fn extend(&mut self, src: &Self) {
        self.unclassified.extend(src.unclassified.clone());
        self.standard.extend(src.standard.clone());
        self.cxx.extend(src.cxx.clone());
        self.qt.extend(src.qt.clone());
    }

    /// Replace generic idents with concrete types they are instantiated (e.g. T -> [i32, i64, f32,..])
    pub fn substitute_generic_insts<'a>(&mut self, from_ident: &syn::Ident, mut to_paths: impl Iterator<Item = &'a syn::Type>) -> syn::Result<()> {
        let from_path = ident_to_path(from_ident.clone());
        if self.remove_unclassified(&from_path) {
            to_paths.try_for_each(|ty| {
                let path = path_from_type(ty)?;
                self.collect_from_path(path)
            })?
        }

        Ok(())
    }
}

struct Visitor<'a> {
    tokens: &'a mut TypeTokens,
    last_error: syn::Result<()>,
}

impl<'a> Visitor<'a> {
    fn new(tokens: &'a mut TypeTokens) -> Self {
        Self {
            tokens,
            last_error: Ok(())
        }
    }

    fn check_result(&mut self, res: syn::Result<()>) {
        if let Err(err) = res {
            self.last_error = Err(err)
        }
    }

    fn result(&self) -> syn::Result<()> {
        self.last_error.clone()
    }

    fn do_visit_path(&mut self, src: &'a syn::Path) -> syn::Result<()> {
        self.result()?; // Early exit if error occurred previously

        // Discard "Self" type early
        if src.segments.first().is_some_and(|first_seg| first_seg.ident == "Self") {
            return Ok(())
        }

        // If it is path consisting of single segment without generic arguments
        // and its ident is generic type ident (e.g. T, K, V, etc)
        if let Some(ident) = src.get_ident() &&
            self.tokens.generic_idents().contains(ident) {
                return Ok(())
        }

        let maybe_type = if are_all_args_generic_idents(src, self.tokens.generic_idents()) {
            let src_ident = get_ident_of_last_path_segment_or_err(src)?;
            // If all the generic arguments are generic idents (e.g. T, K, V, ..)
            // then discard args and handle only generic type ident.
            type_registry::Type::find_by_name(&src_ident.to_string())
        } else {
            type_registry::Type::find_by_path(src)
        };

        // Try to find type in registry
        let Some(type_) = maybe_type else {
            // Unrecognized - add to unclassified set and get back to it later maybe
            self.tokens.insert_unclassified(src.clone());
            return Ok(())
        };

        // Add type to corresponding group depending of type category it belongs to
        match type_ {
            type_registry::Type::Standard(standard) => {
                self.tokens.insert_standard(standard.clone());
            }
            type_registry::Type::Cxx(cxx) => {
                self.tokens.insert_cxx(cxx.clone());
            }
            type_registry::Type::Qt(qt) =>
                self.visit_qt_path(qt, src)?
        }

        // Visit nested items if not qt type
        for seg in &src.segments {
            self.visit_path_segment(seg);
        }

        Ok(())
    }

    fn visit_qt_path(&mut self, qt_type: QtType, src: &'a syn::Path) -> syn::Result<()>{
        // Try as Qt generic type
        if let QtType::GenericWithArgs(generic_w_args) = qt_type {
            // Get generic arguments
            if let Some(_monomorphed) = generic_w_args.get_monomorphed_type() {
                // Monomorphed form is known - then add generic
                self.tokens.insert_qt(QtTypeSpanned::new(generic_w_args.into(), src.span()));
            }
            else if are_all_args_generic_idents(src, self.tokens.generic_idents()) {
                let generic_wo_args = generic_w_args.get_generic_type();
                self.tokens.insert_qt(QtTypeSpanned::new(generic_wo_args.into(), src.span()));
            }
            else {
                self.tokens.insert_unclassified(src.clone());
            }
        }
        else {
            self.tokens.insert_qt(QtTypeSpanned::new(qt_type, src.span()));
        }

        Ok(())
    }

}

impl<'a> Visit<'a> for Visitor<'a> {
    fn visit_path(&mut self, src: &'a syn::Path) {
        let res = self.do_visit_path(src);
        self.check_result(res);
    }

    fn visit_trait_bound(&mut self, _src: &'a syn::TraitBound) {
        // Traits in types are not supported. Do not visit nested syn::Path
    }
}
