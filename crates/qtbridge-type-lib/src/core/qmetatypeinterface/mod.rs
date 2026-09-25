// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/core/qmetatypeinterface/cpp/qmetatypeinterface.h");
        #[namespace = "QtPrivate"]
        type QMetaTypeInterface = super::QMetaTypeInterface;
    }

    #[namespace = "rust::bridge::qmetatypeinterface"]
    unsafe extern "C++" {
        /// Builds an interface from the given layout, flags, name and callbacks.
        ///
        /// `name` is copied and leaked in C++, as Qt keeps it for the lifetime of the program.
        #[rust_name = "fill_fields"]
        fn fillFields(
            align: usize,
            size: usize,
            flags: u32,
            name: &str,
            meta_obj_fn: usize,
            default_ctr_fn: usize,
            copy_ctr_fn: usize,
            dtor_fn: usize,
        ) -> QMetaTypeInterface;
    }
}

#[doc(hidden)]
/// If a QMetaType is generated based on QMetaTypeInterface it must have a
/// constant address in memory during the whole application run.
#[repr(C)]
pub struct QMetaTypeInterface {
    _revision: MaybeUninit<u16>,
    _alignment: MaybeUninit<u16>,
    _size: MaybeUninit<u32>,
    _flags: MaybeUninit<u32>,
    _type_id: MaybeUninit<i32>,
    _meta_object_fn: MaybeUninit<usize>,
    _name: MaybeUninit<usize>,
    _default_ctr: MaybeUninit<usize>,
    _copy_ctr: MaybeUninit<usize>,
    _move_ctr: MaybeUninit<usize>,
    _dtor: MaybeUninit<usize>,
    _equals: MaybeUninit<usize>,
    _less_than: MaybeUninit<usize>,
    _debug_stream: MaybeUninit<usize>,
    _data_stream_out: MaybeUninit<usize>,
    _data_stream_in: MaybeUninit<usize>,
    _legacy_register_op: MaybeUninit<usize>,
}

unsafe impl cxx::ExternType for QMetaTypeInterface {
    type Id = cxx::type_id!("QtPrivate::QMetaTypeInterface");
    type Kind = cxx::kind::Trivial;
}

pub use ffi::fill_fields;
