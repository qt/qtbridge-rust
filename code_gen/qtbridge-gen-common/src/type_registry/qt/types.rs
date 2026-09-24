// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::hash::Hash;

use proc_macro2::Span;
use syn::spanned::Spanned;

use crate::type_registry::type_traits::{FindType, MetaTypeId, TypeInfo, TypeName, TypesEnum};
use crate::type_to_string::path_to_string_fallback;
use crate::type_utils::{get_angle_bracketed_generic_arguments_of_last_path_segment, get_ident_of_last_path_segment_or_err};
use crate::type_registry::qt;
use crate::type_registry::qt::generic::QtGenericArg;
use qt::non_generic::QtNonGenericType;
use qt::generic::{QtGenericTypeWithoutArgs, QtGenericTypeWithArgs};
use qt::monomorphed::QtMonomorphedType;
use qt::monomorphed_alias::QtAliasToMonomorphedType;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum QtType {
    NonGeneric(QtNonGenericType),
    GenericWithoutArgs(QtGenericTypeWithoutArgs),
    GenericWithArgs(QtGenericTypeWithArgs),
    GenericMonomorphed(QtMonomorphedType),
    AliasToMonomorphed(QtAliasToMonomorphedType),
}


type QtTypeMap = BTreeMap<String, QtType>;
thread_local!(static QT_TYPE_MAP: RefCell<QtTypeMap> = RefCell::default());


// Functionality of QtType that is used for 'no_types' case only (type generation).
impl QtType {

    pub fn add_concrete(concrete: QtNonGenericType) {
        QT_TYPE_MAP.with_borrow_mut(|map| {
            map.insert(concrete.name().into(), concrete.into());
        })
    }

    pub fn add_generic(generic: QtGenericTypeWithoutArgs) {
        QT_TYPE_MAP.with_borrow_mut(|map| {
            map.insert(generic.name().to_string(), generic.into());
        })
    }

    pub fn add_monomorphed(monomorphed: QtMonomorphedType) {
        QT_TYPE_MAP.with_borrow_mut(|map| {
            map.insert(monomorphed.name().to_string(), monomorphed.into());
        });
    }

    pub fn try_add_monomorphed(monomorphed_name: String, generic_ident: &syn::Ident, generic_args: Vec<QtGenericArg>, path_in_gen: String, metatypeid: MetaTypeId) -> syn::Result<()> {
        let generic_wo_args = QtGenericTypeWithoutArgs::find_by_name(&generic_ident.to_string())
            .ok_or_else(|| syn::Error::new(generic_ident.span(), format!("Failed to find generic type '{generic_ident}' for monomorphed '{monomorphed_name}'")))?;
        let generic_w_args = generic_wo_args.set_args(generic_args)
            .map_err(|err| syn::Error::new(generic_ident.span(), format!("Failed to set arguments to generic struct '{generic_ident}': {err}")))?;

        Self::add_monomorphed(QtMonomorphedType::new(monomorphed_name, path_in_gen, generic_w_args, metatypeid));
        Ok(())
    }

    pub fn add_alias_to_monomoprhed(alias_monomorphed: QtAliasToMonomorphedType) {
        QT_TYPE_MAP.with_borrow_mut(|map| {
            map.insert(alias_monomorphed.name().to_string(), alias_monomorphed.into());
        });
    }
}

impl QtType {
    /// Iterate over all the values in the map with Qt types
    /// and invoke the provided functor for each value.
    pub fn visit_all<F>(mut f: F) -> Result<(), String>
    where F: FnMut(&QtType) -> Result<(), String>
    {
        QT_TYPE_MAP.with_borrow(|map| -> Result<(), String> {
            for ty in map.values() {
                f(ty)?;
            }
            Ok(())
        })
    }

    pub fn path_in_gen(&self) -> &str {
        match self {
            QtType::NonGeneric(concrete) => concrete.path_in_gen(),
            QtType::GenericWithoutArgs(generic_wo_args) => generic_wo_args.path_in_gen(),
            QtType::GenericWithArgs(generic_w_args) => generic_w_args.path_in_gen(),
            QtType::GenericMonomorphed(monomorphed) => monomorphed.path_in_gen(),
            QtType::AliasToMonomorphed(alias_to_monomorphed) => alias_to_monomorphed.path_in_gen(),
        }
    }
}

impl FindType for QtType {
    fn find_by_name(name: &str) -> Option<Self> {
        QT_TYPE_MAP.with_borrow(|map| map.get(name).cloned())
    }

    fn find_by_path(path: &syn::Path) -> Option<Self> {
        get_qt_type_by_path(path)
            .ok()
            .flatten()
    }

    fn find_by_path_checked(path: &syn::Path) -> syn::Result<Self> {
        get_qt_type_by_path(path)?
            .ok_or_else(|| syn::Error::new(path.span(), format!("Failed to find type by path '{}'", path_to_string_fallback(path))))
    }
}

fn get_qt_type_by_path(path: &syn::Path) -> syn::Result<Option<QtType>> {
    let last_seg_ident = get_ident_of_last_path_segment_or_err(path)?;
    let last_seg_str = last_seg_ident.to_string();

    let Some(ty) = QtType::find_by_name(&last_seg_str) else {
        return Ok(None)
    };

    // If type is generic with args specified - try to find monomorphed form
    if let QtType::GenericWithoutArgs(qt_generic) = &ty
        && let Some(ab) = get_angle_bracketed_generic_arguments_of_last_path_segment(path) {
            let qt_generic_w_types = qt_generic.set_args_from_syn_generic_args(ab)?;
            return Ok(Some(qt_generic_w_types.into()))
    };

    Ok(Some(ty))
}

impl TypesEnum for QtType {
    fn dyn_type_info(&self) -> &dyn TypeInfo {
        match self {
            QtType::NonGeneric(concrete) => concrete.dyn_type_info(),
            QtType::GenericWithoutArgs(generic_wo_args) => generic_wo_args.dyn_type_info(),
            QtType::GenericWithArgs(generic_w_args) => generic_w_args.dyn_type_info(),
            QtType::GenericMonomorphed(monomorphed) => monomorphed.dyn_type_info(),
            QtType::AliasToMonomorphed(alias_to_monomorphed) => alias_to_monomorphed.dyn_type_info(),
        }
    }

    fn mut_dyn_type_info(&mut self) -> &mut dyn TypeInfo {
        match self {
            QtType::NonGeneric(concrete) => concrete.mut_dyn_type_info(),
            QtType::GenericWithoutArgs(generic_wo_args) => generic_wo_args.mut_dyn_type_info(),
            QtType::GenericWithArgs(generic_w_args) => generic_w_args.mut_dyn_type_info(),
            QtType::GenericMonomorphed(monomorphed) => monomorphed.mut_dyn_type_info(),
            QtType::AliasToMonomorphed(alias_to_monomorphed) => alias_to_monomorphed.mut_dyn_type_info(),
        }
    }
}

impl From<QtNonGenericType> for QtType {
    fn from(value: QtNonGenericType) -> Self {
        Self::NonGeneric(value)
    }
}

impl From<QtGenericTypeWithoutArgs> for QtType {
    fn from(value: QtGenericTypeWithoutArgs) -> Self {
        Self::GenericWithoutArgs(value)
    }
}

impl From<QtGenericTypeWithArgs> for QtType {
    fn from(value: QtGenericTypeWithArgs) -> Self {
        Self::GenericWithArgs(value)
    }
}

impl From<QtMonomorphedType> for QtType {
    fn from(value: QtMonomorphedType) -> Self {
        Self::GenericMonomorphed(value)
    }
}

impl From<QtAliasToMonomorphedType> for QtType {
    fn from(value: QtAliasToMonomorphedType) -> Self {
        Self::AliasToMonomorphed(value)
    }
}


#[derive(Clone)]
pub struct QtTypeSpanned {
    ty: QtType,
    span: Span,
}

impl QtTypeSpanned {
    pub fn new(ty: QtType, span: Span) -> Self {
        Self { ty, span }
    }

    pub fn get_type(&self) -> &QtType {
        &self.ty
    }

    pub fn set_type(&mut self, ty: QtType) {
        self.ty = ty;
    }

    pub fn dyn_type_info(&self) -> &dyn TypeInfo {
        self.ty.dyn_type_info()
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl PartialEq for QtTypeSpanned {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
    }
}

impl Eq for QtTypeSpanned {}

impl Hash for QtTypeSpanned {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ty.hash(state);
    }
}
