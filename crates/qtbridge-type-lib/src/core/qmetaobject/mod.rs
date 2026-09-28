// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#[cxx::bridge]
mod ffi {
    #[allow(clippy::missing_safety_doc)]
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/core/qmetaobject/cpp/qmetaobject.h");
        /// The QMetaObject struct contains meta-information about Qt objects.
        ///
        /// See also: [QMetaObject documentation](https://doc.qt.io/qt-6/qmetaobject.html).
        type QMetaObject;
        include!("qtbridge-type-lib/src/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = crate::QMetaType;

        /// Returns the metatype corresponding to this metaobject.
        #[cxx_name = "metaType"]
        fn meta_type(self: &QMetaObject) -> QMetaType;

        #[cxx_name = "inherits"]
        unsafe fn inherits_ptr(self: &QMetaObject, base: *const QMetaObject) -> bool;
    }
}

pub use ffi::QMetaObject;

impl QMetaObject {
    /// Returns `true` if the class described by this QMetaObject
    /// inherits the type described by `base`.
    pub fn inherits(&self, base: &QMetaObject) -> bool {
        unsafe { self.inherits_ptr(base) }
    }
}
