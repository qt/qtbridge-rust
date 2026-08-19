// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]

use qtbridge_type_lib::{QGuiApplication, QQmlApplicationEngine, QVariantMap, QString};
use qtbridge::{qobject, QObjectHolder, QmlRegister};

#[qobject]
pub mod widget {
    #[derive(Default)]
    pub struct Widget {
        pub rust_ran: bool,
    }
    impl Widget {
        // overridden in qml
        #[qslot(qml_name = "ping")]
        fn ping(&mut self) {
            self.rust_ran = true;
        }
    }
}
pub use widget::Widget;

#[qobject]
pub mod registry {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Default)]
    pub struct Registry {
        pub widget: Option<Rc<RefCell<crate::Widget>>>,
        pub qml_ran: bool,
    }
    impl Registry {
        #[qslot(qml_name = "registerWidget")]
        fn register_widget(&mut self, w: Rc<RefCell<crate::Widget>>) {
            self.widget = Some(w);
        }
        #[qslot(qml_name = "qmlPingRan")]
        fn qml_ping_ran(&mut self) {
            self.qml_ran = true;
        }
    }
}
pub use registry::Registry;

const QML: &str = r#"
    import QtQuick
    import tst_qobject_macro
    Item {
        required property var registry
        Widget {
            id: w
            function ping() { registry.qmlPingRan(); }
        }
        Component.onCompleted: registry.registerWidget(w);
    }
"#;

#[cfg(not(miri))]
fn main() {
    let app = QGuiApplication::new();
    Widget::register();

    let registry = Registry::default_with_attached_qobject();
    let registry_var = registry.borrow().as_qvariant();

    let mut props = QVariantMap::default();
    props.insert(QString::from("registry"), registry_var);

    let mut engine = QQmlApplicationEngine::new();
    engine.pin_mut().set_initial_properties(&props);
    engine.pin_mut().load_data(&QML.into(), &Default::default());

    // A QML-extended instance carries a derived QML meta-object; passing it
    // back into Rust must succeed
    let captured = registry.borrow().widget.clone();
    let w = captured.expect("QML-extended Widget should round-trip back into Rust");

    // The QML `function ping` is the most-derived member named "ping", so
    // invoking "ping" resolves to it (not the Rust slot) - the same way a QML
    // caller would resolve it.
    let invoker = w.borrow().get_qml_method_invoker();
    invoker.invoke_method("ping");
    app.process_events();
    app.process_events();
    assert!(registry.borrow().qml_ran,
        "expected the QML function ping() to run (most-derived)");
    assert!(!w.borrow().rust_ran,
        "expected the Rust ping() slot to be shadowed by the QML function");
}

#[cfg(miri)]
fn main() {}
