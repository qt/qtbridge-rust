// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::rc::Rc;
use std::cell::RefCell;
use std::any::TypeId;
use std::collections::HashMap;
use qtbridge_type_lib::{QMetaTypeInterface, QMetaTypeFlag, QMetaObject, QObject};
use crate::qproxies::QCppProxy;
use crate::{QObjectHolder, QMetaInfo};
use crate::qproxies::ConstructionMode;

fn monomorphize_meta_object_fn<T: QObjectHolder>() -> extern "C" fn(*const QMetaTypeInterface) -> *mut QMetaObject {
    extern "C" fn meta_object_fn<T: QObjectHolder>(_iface: *const QMetaTypeInterface) -> *mut QMetaObject {
        let meta_obj_data =
        <T as QMetaInfo>::get_shared_dynamic_meta_object_data();
        meta_obj_data.get_meta_object()
    }
    meta_object_fn::<T>
}

fn monomorphize_default_ctor<T: QObjectHolder>() -> extern "C" fn(*const QMetaTypeInterface, *mut u8) {
    extern "C" fn default_ctor<T: QObjectHolder>(_iface: *const QMetaTypeInterface, addr: *mut u8) {
        let instance =
        Rc::new(RefCell::new(<T as Default>::default()));
        <T as QObjectHolder>::register_instance_in_map(instance, ConstructionMode::AtAddress(addr));
    }
    default_ctor::<T>
}

fn monomorphize_dtor<T: QObjectHolder>() -> extern "C" fn (*const QMetaTypeInterface, *mut u8)  {
    extern "C" fn dtor<T: QObjectHolder>(_iface: *const QMetaTypeInterface, obj: *mut u8) {
        QObject::destruct(obj.cast());
    }
    dtor::<T>
}

/// Builds the [`QMetaTypeInterface`] describing `T`.
pub fn init_interface_for<T: QObjectHolder + 'static>()-> QMetaTypeInterface {
    let flags: u32 =
        (QMetaTypeFlag::NeedsConstruction as u32)
        | (QMetaTypeFlag::NeedsDestruction as u32)
        | (QMetaTypeFlag::NeedsCopyConstruction as u32)
        | (QMetaTypeFlag::NeedsMoveConstruction as u32)
        | (QMetaTypeFlag::PointerToQObject as u32);

    let class_name = std::ffi::CString::new(std::any::type_name::<T>())
        .expect("CString::new failed")
        .into_bytes_with_nul()
        .leak();

    QMetaTypeInterface::fill_fields(
        <<T as QMetaInfo>::CppProxy as QCppProxy>::get_align(),
        <<T as QMetaInfo>::CppProxy as QCppProxy>::get_size(),
        flags,
        class_name,
        monomorphize_meta_object_fn::<T>() as usize,
        monomorphize_default_ctor::<T>() as usize,
        0,
        monomorphize_dtor::<T>() as usize,
    )
}

/// Builds the [`QMetaTypeInterface`] describing `T*` - a pointer to the
/// QObject-derived type `T`.
pub fn init_ptr_interface_for<T: QObjectHolder + 'static>() -> QMetaTypeInterface {
    let flags: u32 =
        (QMetaTypeFlag::IsPointer as u32)
        | (QMetaTypeFlag::PointerToQObject as u32)
        | (QMetaTypeFlag::RelocatableType as u32);

    let name = std::ffi::CString::new(format!("{}*", std::any::type_name::<T>()))
        .expect("CString::new failed")
        .into_bytes_with_nul()
        .leak();

    QMetaTypeInterface::fill_fields(
        std::mem::align_of::<*mut QObject>(),
        std::mem::size_of::<*mut QObject>(),
        flags,
        name,
        monomorphize_meta_object_fn::<T>() as usize,
        0,
        0,
        0,
    )
}

/// HashMap cached [`QMetaTypeInterface`], used for generic QObject types
/// where a per-instantiation `OnceLock` is not available.
pub fn interface_for_generic<T: QObjectHolder + 'static>() -> &'static QMetaTypeInterface {
    thread_local!(static IFACE_MAP: RefCell<HashMap<TypeId , *const QMetaTypeInterface>> = RefCell::new(HashMap::new ()));
    let type_id = TypeId::of::<T>();
    {
        let iface_ptr = IFACE_MAP
            .with_borrow(|iface_map| iface_map.get(&type_id).copied().unwrap_or_default());
        if let Some(iface_ref) = unsafe { iface_ptr.as_ref() } {
            return iface_ref;
        }
    }
    let iface_ref = Box::leak(Box::new(init_interface_for::<T>()));
    let iface_ptr = std::ptr::from_ref(iface_ref);
    IFACE_MAP.with_borrow_mut(|iface_map| iface_map.insert(type_id, iface_ptr));
    iface_ref
}

/// Pointer-metatype counterpart of [`interface_for_generic`], used for generic
/// QObject types where a per-instantiation `OnceLock` is not available.
pub fn ptr_interface_for_generic<T: QObjectHolder + 'static>() -> &'static QMetaTypeInterface {
    thread_local!(static PTR_IFACE_MAP: RefCell<HashMap<TypeId, *const QMetaTypeInterface>> = RefCell::new(HashMap::new()));
    let type_id = TypeId::of::<T>();
    {
        let iface_ptr = PTR_IFACE_MAP
            .with_borrow(|iface_map| iface_map.get(&type_id).copied().unwrap_or_default());
        if let Some(iface_ref) = unsafe { iface_ptr.as_ref() } {
            return iface_ref;
        }
    }
    let iface_ref = Box::leak(Box::new(init_ptr_interface_for::<T>()));
    let iface_ptr = std::ptr::from_ref(iface_ref);
    PTR_IFACE_MAP.with_borrow_mut(|iface_map| iface_map.insert(type_id, iface_ptr));
    iface_ref
}
