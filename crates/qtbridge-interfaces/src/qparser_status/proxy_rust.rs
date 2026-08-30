// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_runtime::{DispatchMetaCall, QObjectHolder};
use crate::call_rust_trait_impl;
use crate::genericrustproxy::GenericRustProxy;
use super::proxy_cpp_bridge::QParserStatusProxyCpp;

/// A trait for hooking into QML component construction stages.
///
/// [`QParserStatus::class_begin`] is called first when the
/// object is created, followed by [`QParserStatus::component_complete`] once
/// all QML properties have been set. This allows you to defer expensive
/// initialization until the component is fully constructed.
///
/// This trait is the Rust equivalent of
/// [`QQmlParserStatus`](https://doc.qt.io/qt-6/qqmlparserstatus.html).
///
/// This trait requires the `qobject` macro with `Base = QParserStatus` to set
/// up the correct Qt proxy.
///
/// ## Example
///
/// ```rust
/// use qtbridge::{QApp, qobject};
///
/// #[qobject(Base = QParserStatus)]
/// pub mod backend {
///     use qtbridge::QParserStatus;
///
///     #[derive(Default)]
///     pub struct Status {
///         somewhat_ready: bool,
///     }
///
///     impl Status {
///         #[qsignal]
///         fn ready(&mut self);
///     }
///
///     impl QParserStatus for Status {
///         fn class_begin(&mut self) {
///             self.somewhat_ready = true;
///         }
///
///         fn component_complete(&mut self) {
///             if self.somewhat_ready {
///                 self.ready();
///             }
///         }
///     }
/// }
///
/// const QML_CODE: &str =
/// r#"
///     import QtQuick
///     import QtQuick.Controls
///#     import qtbridge_interfaces
///     // import your module
///
///     ApplicationWindow {
///         visible: true
///         Status {
///             onReady: closeTimer.start()
///         }
///         Timer {
///             id: closeTimer
///             interval: 1
///             onTriggered: Qt.quit()
///         }
///     }
/// "#;
///
/// fn main() {
///     QApp::new()
///         .register::<backend::Status>()
///         .load_qml(QML_CODE.as_bytes())
///         .run();
/// }
/// ```
pub trait QParserStatus {
    /// Called when the object is first created, before any properties are set.
    ///
    /// Use this to perform any initialization that must happen before
    /// properties are applied.
    fn class_begin(&mut self) {}

    /// Called after all QML properties have been set.
    ///
    /// Use this to perform initialization that depends on property values,
    /// or to emit signals that notify QML the object is ready.
    fn component_complete(&mut self) {}
}

#[doc(hidden)]
pub trait QParserStatusAdapter: QParserStatus + DispatchMetaCall { }

impl<T> QParserStatusAdapter for T
where T: QParserStatus + QObjectHolder<ProxyRust = QParserStatusProxyRust> { }

pub type QParserStatusProxyRust = GenericRustProxy<QParserStatusProxyCpp, dyn QParserStatusAdapter>;

impl QParserStatusProxyRust {
    pub fn class_begin(&mut self) {
        call_rust_trait_impl!(mut self, class_begin())
    }

    pub fn component_complete(&mut self) {
        call_rust_trait_impl!(mut self, component_complete())
    }
}
