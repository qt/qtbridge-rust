// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_rust::QParserStatusProxyRust;
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qvariant/cpp/qvariant.h");
        type QVariant = qtbridge_type_lib::QVariant;
    }

    extern "Rust" {
        type QParserStatusProxyRust;
        # [Self = QParserStatusProxyRust]
        # [cxx_name = dropSelf]
        unsafe fn drop_self(self_ptr: *mut QParserStatusProxyRust);

        # [cxx_name = invokeSlot]
        fn invoke_slot(&self, slot_id: u32, args: &[*const u8], outputs: &[*mut u8]);
        # [cxx_name = invokeSlotMut]
        fn invoke_slot_mut(&self, slot_id: u32, args: &[*const u8], outputs: &[*mut u8]);
        # [cxx_name = readProperty]
        fn read_property(&self, prop_id: u32) -> QVariant;
        # [cxx_name = writeProperty]
        fn write_property(&self, prop_id: u32, value: &QVariant);

        fn class_begin(&mut self);
        fn component_complete(&mut self);
    }
}
unsafe impl cxx::ExternType for QParserStatusProxyRust {
    type Id = cxx::type_id!(QParserStatusProxyRust);
    type Kind = cxx::kind::Trivial;
}
