// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_runtime::qproxies::QCppProxy;
use qtbridge_runtime::DynamicMetaObjectData;
use qtbridge_type_lib::QMetaObject;
use crate::impl_qcpp_proxy;

use super::proxy_rust::QTableModelProxyRust;
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/core/qhash/cpp/qhash_i32_qbytearray.h");
        type QHash_i32_QByteArray = qtbridge_type_lib::QHash_i32_QByteArray;
        include!("qtbridge-type-lib/src/core/qmetaobject/cpp/qmetaobject.h");
        type QMetaObject = qtbridge_type_lib::QMetaObject;
        include!("qtbridge-type-lib/src/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = qtbridge_type_lib::QMetaType;
        include!("qtbridge-runtime/src/cpp/dynamicmetaobjectdata.h");
        type DynamicMetaObjectData = qtbridge_runtime::DynamicMetaObjectData;
        include!("qtbridge-type-lib/src/core/qmodelindex/cpp/qmodelindex.h");
        type QModelIndex = qtbridge_type_lib::QModelIndex;
        include!("qtbridge-type-lib/src/core/qvariant/cpp/qvariant.h");
        type QVariant = qtbridge_type_lib::QVariant;
        include!("qtbridge-interfaces/src/qtable_model/proxy_rust_bridge.rs.h");
        type QTableModelProxyRust = super::QTableModelProxyRust;
    }
    #[allow(clippy::missing_safety_doc)]
    #[namespace = "rust::bridge"]
    unsafe extern "C++" {
        include!("qtbridge-interfaces/src/qtable_model/cpp/QTableModelProxyCpp.h");
        type QTableModelProxyCpp;
        # [Self = QTableModelProxyCpp]
        # [rust_name = create]
        unsafe fn create(rust_proxy: *mut QTableModelProxyRust, metaobject: *const DynamicMetaObjectData) -> *mut QTableModelProxyCpp;
        # [Self = QTableModelProxyCpp]
        # [rust_name = create_at]
        unsafe fn createAt(rust_proxy: *mut QTableModelProxyRust, metaobject: *const DynamicMetaObjectData, addr: *mut u8) -> *mut QTableModelProxyCpp;
        # [Self = QTableModelProxyCpp]
        # [rust_name = static_qmeta_object]
        fn baseStaticMetaObject() -> &'static QMetaObject;
        # [Self = QTableModelProxyCpp]
        # [rust_name = size_of]
        fn sizeOfProxy() -> usize;
        # [Self = QTableModelProxyCpp]
        # [rust_name = align_of]
        fn alignOfProxy() -> usize;
        # [Self = QTableModelProxyCpp]
        # [rust_name = parser_status_cast]
        fn parserStatusCast() -> i32;
        # [rust_name = emit_signal_cpp]
        fn emitSignal(self: Pin<&mut Self>, signal_name: &str, argv: &[*const u8]);
        # [rust_name = base_role_names]
        fn base_roleNames(&self) -> QHash_i32_QByteArray;
        # [rust_name = base_set_data]
        fn base_setData(self: Pin<&mut Self>, index: &QModelIndex, value: &QVariant, role: i32) -> bool;
        # [rust_name = base_remove_columns]
        fn base_removeColumns(self: Pin<&mut Self>, first: i32, count: i32, parent: &QModelIndex) -> bool;
        # [rust_name = base_remove_rows]
        fn base_removeRows(self: Pin<&mut Self>, first: i32, count: i32, parent: &QModelIndex) -> bool;
        # [rust_name = base_sibling]
        fn base_sibling(&self, row: i32, column: i32, idx: &QModelIndex) -> QModelIndex;
        # [rust_name = base_data_changed]
        fn dataChanged(self: Pin<&mut Self>, top_left: &QModelIndex, bottom_right: &QModelIndex);
        # [rust_name = base_begin_insert_columns]
        fn beginInsertColumns(self: Pin<&mut Self>, parent: &QModelIndex, first: i32, last: i32);
        # [rust_name = base_end_insert_columns]
        fn endInsertColumns(self: Pin<&mut Self>);
        # [rust_name = base_begin_insert_rows]
        fn beginInsertRows(self: Pin<&mut Self>, parent: &QModelIndex, first: i32, last: i32);
        # [rust_name = base_end_insert_rows]
        fn endInsertRows(self: Pin<&mut Self>);
        # [rust_name = base_begin_move_columns]
        fn beginMoveColumns(
            self: Pin<&mut Self>,
            source_parent: &QModelIndex,
            source_first: i32,
            source_last: i32,
            destination_parent: &QModelIndex,
            destination_child: i32,
        );
        # [rust_name = base_end_move_columns]
        fn endMoveColumns(self: Pin<&mut Self>);
        # [rust_name = base_begin_move_rows]
        fn beginMoveRows(
            self: Pin<&mut Self>,
            source_parent: &QModelIndex,
            source_first: i32,
            source_last: i32,
            destination_parent: &QModelIndex,
            destination_child: i32,
        );
        # [rust_name = base_end_move_rows]
        fn endMoveRows(self: Pin<&mut Self>);
        # [rust_name = base_begin_remove_columns]
        fn beginRemoveColumns(self: Pin<&mut Self>, parent: &QModelIndex, first: i32, last: i32);
        # [rust_name = base_end_remove_columns]
        fn endRemoveColumns(self: Pin<&mut Self>);
        # [rust_name = base_begin_remove_rows]
        fn beginRemoveRows(self: Pin<&mut Self>, parent: &QModelIndex, first: i32, last: i32);
        # [rust_name = base_end_remove_rows]
        fn endRemoveRows(self: Pin<&mut Self>);
        # [rust_name = base_begin_reset_model]
        fn beginResetModel(self: Pin<&mut Self>);
        # [rust_name = base_end_reset_model]
        fn endResetModel(self: Pin<&mut Self>);
        # [rust_name = base_create_index]
        fn createIndex(&self, row: i32, column: i32, ptr: usize) -> QModelIndex;
    }
}
pub use ffi::QTableModelProxyCpp;

impl_qcpp_proxy!(QTableModelProxyCpp, QTableModelProxyRust);
