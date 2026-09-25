// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QMetaTypeInterface;
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = super::QMetaType;
        include!("qtbridge-type-lib/src/core/qmetatypeinterface/cpp/qmetatypeinterface.h");
        #[namespace = "QtPrivate"]
        type QMetaTypeInterface = crate::QMetaTypeInterface;
    }

    #[namespace = "rust::bridge::qmetatype"]
    unsafe extern "C++" {
        #[rust_name = "default_qmetatype"]
        fn defaultQMetaType() -> QMetaType;
        #[rust_name = "new_with_interface"]
        fn newWithInterface(iface: &QMetaTypeInterface) -> QMetaType;
        fn eq(lhs: &QMetaType, rhs: &QMetaType) -> bool;
        fn id(obj: &QMetaType) -> i32;
        fn name(obj: &QMetaType) -> String;
        #[rust_name = "register_type"]
        fn registerType(obj: &QMetaType);
    }
}

/// The QMetaType struct manages named types in the meta-object system.
///
/// See also: [QMetaType documentation](https://doc.qt.io/qt-6/qmetatype.html).
#[derive(Debug)]
#[repr(C)]
pub struct QMetaType {
    _d_ptr: MaybeUninit<usize>,
}

unsafe impl cxx::ExternType for QMetaType {
    type Id = cxx::type_id!("QMetaType");
    type Kind = cxx::kind::Trivial;
}

impl Default for QMetaType {
    fn default() -> Self {
        ffi::default_qmetatype()
    }
}

impl PartialEq for QMetaType {
    fn eq(&self, other: &Self) -> bool {
        ffi::eq(self, other)
    }
}

#[doc(hidden)]
pub enum QMetaTypeFlag {
    NeedsConstruction = 0x1,
    NeedsDestruction = 0x2,
    RelocatableType = 0x4,
    PointerToQObject = 0x8,
    IsEnumeration = 0x10,
    SharedPointerToQObject = 0x20,
    WeakPointerToQObject = 0x40,
    TrackingPointerToQObject = 0x80,
    IsUnsignedEnumeration = 0x100,
    IsGadget = 0x200,
    PointerToGadget = 0x400,
    IsPointer = 0x800,
    IsQmlList = 0x1000,
    IsConst = 0x2000,
    NeedsCopyConstruction = 0x4000,
    NeedsMoveConstruction = 0x8000,
}

impl QMetaType {
    /// Creates a `QMetaType` instance from the specified `QMetaTypeInterface`.
    pub fn new_with_interface(iface: &'static QMetaTypeInterface) -> Self {
        ffi::new_with_interface(iface)
    }

    /// Returns id type held by this QMetaType instance.
    pub fn id(&self) -> i32 {
        ffi::id(self)
    }

    /// Returns the type name associated with this QMetaType, or an empty string if type is not valid.
    pub fn name(&self) -> String {
        ffi::name(self)
    }

    /// Registers this QMetaType with the type registry so it can be found by name, using QMetaType::fromName().
    pub fn register_type(&self) {
        ffi::register_type(self)
    }
}
