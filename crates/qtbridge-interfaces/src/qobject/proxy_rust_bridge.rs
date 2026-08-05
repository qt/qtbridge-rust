// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_rust::QObjectProxyRust;
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qvariant/cpp/qvariant.h");
        type QVariant = qtbridge_type_lib::QVariant;
    }

    extern "Rust" {
        type QObjectProxyRust;
        # [Self = QObjectProxyRust]
        # [cxx_name = dropSelf]
        unsafe fn drop_self(self_ptr: *mut QObjectProxyRust);

        # [cxx_name = invokeSlot]
        fn invoke_slot(&self, slot_id: u32, args: &[*const u8], outputs: &[*mut u8]);
        # [cxx_name = invokeSlotMut]
        fn invoke_slot_mut(&self, slot_id: u32, args: &[*const u8], outputs: &[*mut u8]);
        # [cxx_name = readProperty]
        fn read_property(&self, prop_id: u32) -> QVariant;
        # [cxx_name = writeProperty]
        fn write_property(&self, prop_id: u32, value: &QVariant);
    }
}
unsafe impl cxx::ExternType for QObjectProxyRust {
    type Id = cxx::type_id!(QObjectProxyRust);
    type Kind = cxx::kind::Trivial;
}
