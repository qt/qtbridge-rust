// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use super::proxy_rust::QAbstractItemModelProxyRust;
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/generated/core/qhash/cpp/qhash_i32_qbytearray.h");
        type QHash_i32_QByteArray = qtbridge_type_lib::QHash_i32_QByteArray;
        include!("qtbridge-type-lib/src/generated/core/qmodelindex/cpp/qmodelindex.h");
        type QModelIndex = qtbridge_type_lib::QModelIndex;
        include!("qtbridge-type-lib/src/generated/core/qvariant/cpp/qvariant.h");
        type QVariant = qtbridge_type_lib::QVariant;
    }
    extern "Rust" {
        type QAbstractItemModelProxyRust;
        # [Self = QAbstractItemModelProxyRust]
        # [cxx_name = dropSelf]
        unsafe fn drop_self(self_ptr: *mut QAbstractItemModelProxyRust);
        # [cxx_name = index]
        fn index(&self, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex;
        # [cxx_name = parent]
        fn parent(&self, child: &QModelIndex) -> QModelIndex;
        # [cxx_name = rowCount]
        fn row_count(&self, parent: &QModelIndex) -> i32;
        # [cxx_name = columnCount]
        fn column_count(&self, parent: &QModelIndex) -> i32;
        # [cxx_name = data]
        fn data(&self, index: &QModelIndex, role: i32) -> QVariant;
        # [cxx_name = roleNames]
        fn role_names(&self) -> QHash_i32_QByteArray;
        # [cxx_name = setData]
        fn set_data(&mut self, index: &QModelIndex, value: &QVariant, role: i32) -> bool;
        # [cxx_name = removeRows]
        fn remove_rows(&mut self, first: i32, count: i32, parent: &QModelIndex) -> bool;
        # [cxx_name = sibling]
        fn sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex;

        # [cxx_name = invokeSlot]
        fn invoke_slot(&self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]);
        # [cxx_name = invokeSlotMut]
        fn invoke_slot_mut(&self, slot_id: u32, inputs: &[*const u8], outputs: &[*mut u8]);
        # [cxx_name = readProperty]
        fn read_property(&self, prop_id: u32) -> QVariant;
        # [cxx_name = writeProperty]
        fn write_property(&self, prop_id: u32, value: &QVariant);
    }
}
unsafe impl cxx::ExternType for QAbstractItemModelProxyRust {
    type Id = cxx::type_id!(QAbstractItemModelProxyRust);
    type Kind = cxx::kind::Trivial;
}
