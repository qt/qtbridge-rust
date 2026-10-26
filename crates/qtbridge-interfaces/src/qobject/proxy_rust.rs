// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_runtime::{DispatchMetaCall, QObjectHolder};
use crate::genericrustproxy::GenericRustProxy;
use super::proxy_cpp_bridge::QObjectProxyCpp;

/* QObject trait left out on purpose */

pub trait QObjectAdapter: DispatchMetaCall {}

impl<T> QObjectAdapter for T
where T: QObjectHolder<ProxyRust = QObjectProxyRust> {}

pub type QObjectProxyRust = GenericRustProxy<QObjectProxyCpp, dyn QObjectAdapter>;

// SAFETY: `upcast` is an unsizing coercion of the same allocation.
unsafe impl<T: QObjectAdapter + 'static> qtbridge_runtime::qproxies::AdapterUpcast<T> for QObjectProxyRust {
    fn upcast(rc: std::rc::Rc<std::cell::RefCell<T>>) -> std::rc::Rc<std::cell::RefCell<dyn QObjectAdapter>> {
        rc
    }
}
