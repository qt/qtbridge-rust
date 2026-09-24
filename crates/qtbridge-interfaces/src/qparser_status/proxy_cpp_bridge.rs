// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_runtime::qproxies::QCppProxy;
use qtbridge_runtime::DynamicMetaObjectData;
use qtbridge_type_lib::QMetaObject;
use super::proxy_rust::QParserStatusProxyRust;
use crate::impl_qcpp_proxy;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("qtbridge-type-lib/src/core/qmetaobject/cpp/qmetaobject.h");
        type QMetaObject = qtbridge_type_lib::QMetaObject;
        include!("qtbridge-type-lib/src/core/qmetatype/cpp/qmetatype.h");
        type QMetaType = qtbridge_type_lib::QMetaType;
        include!("qtbridge-runtime/src/cpp/dynamicmetaobjectdata.h");
        type DynamicMetaObjectData = qtbridge_runtime::DynamicMetaObjectData;
        include!("qtbridge-interfaces/src/qparser_status/proxy_rust_bridge.rs.h");
        type QParserStatusProxyRust = super::QParserStatusProxyRust;
    }
    #[allow(clippy::missing_safety_doc)]
    #[namespace = "rust::bridge"]
    unsafe extern "C++" {
        include!("qtbridge-interfaces/src/qparser_status/cpp/QParserStatusProxyCpp.h");
        type QParserStatusProxyCpp;
        # [Self = QParserStatusProxyCpp]
        # [rust_name = create]
        unsafe fn create(rust_proxy: *mut QParserStatusProxyRust, metaobject: *const DynamicMetaObjectData) -> *mut QParserStatusProxyCpp;
        # [Self = QParserStatusProxyCpp]
        # [rust_name = create_at]
        unsafe fn createAt(rust_proxy: *mut QParserStatusProxyRust, metaobject: *const DynamicMetaObjectData, addr: *mut u8)
        -> *mut QParserStatusProxyCpp;
        # [Self = QParserStatusProxyCpp]
        # [rust_name = static_qmeta_object]
        fn baseStaticMetaObject() -> &'static QMetaObject;
        # [Self = QParserStatusProxyCpp]
        # [rust_name = size_of]
        fn sizeOfProxy() -> usize;
        # [Self = QParserStatusProxyCpp]
        # [rust_name = align_of]
        fn alignOfProxy() -> usize;
        # [Self = QParserStatusProxyCpp]
        # [rust_name = parser_status_cast]
        fn parserStatusCast() -> i32;
        # [rust_name = emit_signal_cpp]
        fn emitSignal(self: Pin<&mut Self>, signal_name: &str, argv: &[*const u8]);
    }
}
pub use ffi::QParserStatusProxyCpp;

impl_qcpp_proxy!(QParserStatusProxyCpp, QParserStatusProxyRust);
