// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QMetaTypeInterface;
use std::mem::MaybeUninit;
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/core/qmetatype/cpp/qmetatype.h");
        #[allow(dead_code)]
        type QMetaType = super::QMetaType;
        include!("qtbridge-type-lib/src/core/qmetatypeinterface/cpp/qmetatypeinterface.h");
        #[namespace = "QtPrivate"]
        type QMetaTypeInterface = crate::QMetaTypeInterface;
    }
    #[namespace = "rust::bridge::qmetatype"]
    unsafe extern "C++" {
        # [rust_name = qmeta_type_default]
        fn QMetaType_Default() -> QMetaType;
        # [rust_name = qmeta_type_eq]
        fn QMetaType_Eq(lhs: &QMetaType, rhs: &QMetaType) -> bool;
        # [rust_name = inline_cpp_fn_new]
        fn inlineCppFn_new(type_id: i32) -> QMetaType;
        # [rust_name = inline_cpp_fn_new_with_interface]
        unsafe fn inlineCppFn_new_with_interface(iface: *const QMetaTypeInterface) -> QMetaType;
        # [rust_name = inline_cpp_fn_id]
        fn inlineCppFn_id(_obj: &QMetaType) -> i32;
        # [rust_name = inline_cpp_fn_is_valid]
        fn inlineCppFn_is_valid(_obj: &QMetaType) -> bool;
        # [rust_name = inline_cpp_fn_name]
        fn inlineCppFn_name(_obj: &QMetaType) -> String;
        # [rust_name = inline_cpp_fn_register_type]
        fn inlineCppFn_register_type(_obj: &QMetaType);
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
        ffi::qmeta_type_default()
    }
}
impl PartialEq for QMetaType {
    fn eq(&self, other: &Self) -> bool {
        ffi::qmeta_type_eq(self, other)
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
    #[allow(dead_code)]
    /// Constructs a QMetaType object specified by its Id.
    pub fn new(type_id: i32) -> Self {
        ffi::inline_cpp_fn_new(type_id)
    }
    #[allow(dead_code)]
    /// Creates a `QMetaType` instance from the specified `QMetaTypeInterface`.
    pub fn new_with_interface(iface: *const QMetaTypeInterface) -> Self {
        let cpp = ffi::inline_cpp_fn_new_with_interface;
        unsafe { cpp(iface) }
    }
    #[allow(dead_code)]
    /// Returns id type held by this QMetaType instance.
    pub fn id(&self) -> i32 {
        ffi::inline_cpp_fn_id(self)
    }
    #[allow(dead_code)]
    /// Returns true if this QMetaType object contains valid information about a type, false otherwise.
    pub fn is_valid(&self) -> bool {
        ffi::inline_cpp_fn_is_valid(self)
    }
    #[allow(dead_code)]
    /// Returns the type name associated with this QMetaType, or an empty string if type is not valid.
    pub fn name(&self) -> String {
        let cpp = ffi::inline_cpp_fn_name;
        cpp(self)
    }
    #[allow(dead_code)]
    /// Registers this QMetaType with the type registry so it can be found by name, using QMetaType::fromName().
    pub fn register_type(&self) {
        ffi::inline_cpp_fn_register_type(self)
    }
}
