// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::qproxies::QCppProxy;
use crate::{DynamicMetaObjectBuilder, DynamicMetaObjectData};
use qtbridge_type_lib::QMetaType;

#[cfg(doc)]
use qtbridge_type_lib::QMetaObject;

/// Extends a C++ QObject's static meta-object with signals, slots, and properties
/// defined on the Rust side, producing a dynamic meta-object that QMetaObject
/// introspection system can query as if the type were a native QObject subclass.
///
/// The associated [`QCppProxy`] provides the parent static [`QMetaObject`] as well
/// as some other information; implementors (usually qtbridge macros) supply
/// [`Self::build_dynamic_meta_type`] to declare the Rust-side additions and
/// [`Self::get_shared_dynamic_meta_object_data`]
/// to return a cached instance of the built meta-object data. Other functions provide
/// a reasonable default implementation
pub trait QMetaInfo : 'static {
    /// The CppProxy provides the static meta-object
    type CppProxy: QCppProxy;

    /// The class_name in the Qt meta-object system
    fn class_name() -> &'static str {
        std::any::type_name::<Self>()
    }

    /// This function takes the meta_obj_builder and builds a meta object
    /// that contains all slots, signals and properties for the respective type.
    /// This function is usually implemented by a macro.
    fn build_dynamic_meta_type(meta_obj_builder: std::pin::Pin<&mut DynamicMetaObjectBuilder>);

    /// Return DynamicMetaObjectData containing information
    /// about signals/slots/properties for given Rust object.
    fn get_shared_dynamic_meta_object_data() -> &'static DynamicMetaObjectData;

    /// Returns the [`QMetaType`] identifying this type, unique per concrete
    /// type (including per generic instantiation).
    /// This function is usually implemented by a macro.
    fn get_qmetatype() -> QMetaType;

    /// Returns the [`QMetaType`] for a pointer to this type (`Self *`).
    /// This function is usually implemented by a macro.
    fn get_qobject_ptr_qmetatype() -> QMetaType;

    /// Creates a new `DynamicMetaObjectData` object and returns
    /// a raw pointer to the heap-allocated object.
    /// Ownership is not managed internally; the caller is responsible for it.
    fn create_dynamic_meta_object_data_for_type() -> *const DynamicMetaObjectData {
        let mut builder = crate::create_dynamic_meta_object_builder(
            Self::class_name(),
            Self::CppProxy::get_static_meta_object());
        Self::build_dynamic_meta_type(builder.pin_mut());
        builder.pin_mut()
            .take_dynamic_metaobject_data()
    }
}

pub fn dynamic_meta_object_data_for_generic<T: QMetaInfo + ?Sized>() -> &'static DynamicMetaObjectData {
    thread_local!(static DYNAMIC_META_MAP: RefCell<HashMap<TypeId, *const DynamicMetaObjectData>> =
        RefCell::new(HashMap::new()));

    let type_id = TypeId::of::<T>();
    {
        let meta_data_ptr = DYNAMIC_META_MAP.with_borrow(|dynamic_meta_builder_map| {
            dynamic_meta_builder_map.get(&type_id)
                .copied()
                .unwrap_or_default()
        });
        if let Some(meta_data_ref) = unsafe { meta_data_ptr.as_ref() } {
            return meta_data_ref;
        }
    }

    let meta_data_ptr = T::create_dynamic_meta_object_data_for_type();
    let meta_data_ref = unsafe { meta_data_ptr.as_ref() }.unwrap();
    DYNAMIC_META_MAP.with_borrow_mut(|dynamic_meta_builder_map| {
        dynamic_meta_builder_map.insert(type_id, meta_data_ptr);
    });

    meta_data_ref
}
