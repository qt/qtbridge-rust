// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::DynamicMetaObjectData;

#[cxx::bridge]
pub mod ffi {

    #[repr(i8)]
    enum Mutability {
        Immutable,
        Mutable,
    }

    unsafe extern "C++" {
        include!("cpp/dynamicmetaobjectdata.h");
        type DynamicMetaObjectData = super::DynamicMetaObjectData;

        include!("qtbridge-type-lib/src/core/qmetaobject/cpp/qmetaobject.h");
        type QMetaObject = qtbridge_type_lib::QMetaObject;

        include!("qtbridge-type-lib/src/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = qtbridge_type_lib::QMetaType;

        include!("qtbridge-type-lib/src/core/qobject/cpp/qobject.h");
        type QObject = qtbridge_type_lib::QObject;

        include!("qtbridge-type-lib/src/core/qvariant/cpp/qvariant.h");
        type QVariant = qtbridge_type_lib::QVariant;

        include!("cpp/dynamicmetaobjectdata.h");
        type Mutability;
    }

    unsafe extern "C++" {
        include!("cpp/dynamicmetaobjectbuilder.h");
        type DynamicMetaObjectBuilder;

        #[rust_name = "add_class_info"]
        fn addClassInfo(self: Pin<&mut Self>, name: &str, value: &str);

        #[rust_name = "register_property"]
        fn registerProperty(self: Pin<&mut Self>, name: &str, prop_id: u32, meta_type: &QMetaType, is_constant: bool, notify_signal: &str);

        #[rust_name = "register_signal"]
        fn registerSignal(self: Pin<&mut Self>, name: &str, arg_meta_types: &[QMetaType]);

        #[rust_name = "register_slot"]
        fn registerSlot(self: Pin<&mut Self>, name: &str, slot_id: u32, arg_meta_types: &[QMetaType], return_meta_type: &QMetaType, mutability: Mutability);

        #[rust_name = "end_meta_registration"]
        fn endMetaRegistration(self: Pin<&mut Self>);

        #[rust_name = "take_dynamic_metaobject_data"]
        fn takeDynamicMetaObjectData(self: Pin<&mut Self>) -> *const DynamicMetaObjectData;

        #[rust_name = "create_dynamic_meta_object_builder"]
        fn createDynamicMetaObjectBuilder(rust_struct_name: &str, static_meta: &QMetaObject) -> UniquePtr<DynamicMetaObjectBuilder>;
    }
}

//unsafe impl Sync for crate::DynamicMetaObjectBuilder {}

pub use ffi::{DynamicMetaObjectBuilder, Mutability, create_dynamic_meta_object_builder};

