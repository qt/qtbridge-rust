// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! Engine-owned entries in the registry: QML-created objects have no Rust
//! handles, so they sit at the collector's garbage signature (strong count
//! equal to the proxy's own) and only their owner state protects them. Their
//! value lives exactly as long as their `QObject`: the engine's deletion
//! frees it through the proxy's strong reference.

#[cfg(not(miri))]
#[path = "../../common/mod.rs"]
mod common;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use qtbridge::{QApp, QmlElement, qobject};

#[derive(Default)]
pub struct Child {}

#[qobject]
impl Child {
    #[qslot(qml_name = "ping")]
    fn ping(&self) -> i32 {
        42
    }
}

#[derive(Default)]
pub struct Backend {
    pub child_pings: i32,
    pub observed: Weak<RefCell<Child>>,
}

#[qobject]
impl Backend {
    #[qslot(qml_name = "collectGarbage")]
    fn collect_garbage(&mut self) {
        qtbridge::collect_garbage();
    }

    /// Records a weak observer without keeping a handle: the argument is
    /// the only Rust handle and drops when the slot returns.
    #[qslot(qml_name = "observe")]
    fn observe(&mut self, child: Rc<RefCell<Child>>) {
        self.observed = Rc::downgrade(&child);
    }

    #[qslot(qml_name = "reportPing")]
    fn report_ping(&mut self, answer: i32) {
        if answer == 42 {
            self.child_pings += 1;
        }
    }
}

/// A QML-declared object has no Rust handles: its strong count equals the
/// proxy's own, the collector's garbage signature. collect_garbage() must
/// spare it nonetheless and the engine's objects are never ours to delete.
/// (Layered protection: the owner state, and the engine flagging its
/// objects `JavaScriptOwnership`.)
fn engine_created_object_is_never_collected() {
    Child::register();
    let backend = Rc::new(RefCell::new(Backend::default()));

    let mut qapp = QApp::new();
    qapp.set_initial_object("backend", backend.clone())
        .load_qml(br#"
        import QtQuick
        import tst_ownership
        Item {
            required property var backend
            Child { id: child }
            Component.onCompleted: {
                backend.collectGarbage();
                backend.reportPing(child.ping());
            }
        }
    "#);

    assert_eq!(backend.borrow().child_pings, 1,
        "collect_garbage() must not delete an engine-owned object, \
         although its strong count reads like garbage");
}

/// The value of an engine-owned object lives exactly as long as its
/// `QObject`: the engine's deletion drops the proxy's strong reference,
/// which is the last one.
fn engine_deletion_frees_the_value() {
    Child::register();
    let backend = Rc::new(RefCell::new(Backend::default()));

    let mut qapp = QApp::new();
    qapp.set_initial_object("backend", backend.clone())
        .load_qml(br#"
        import QtQuick
        import tst_ownership
        Item {
            id: root
            required property var backend
            Component { id: comp; Child {} }
            Component.onCompleted: {
                let c = comp.createObject(root);
                backend.observe(c);
                // Deferred: the engine deletes the QObject on the next
                // event-loop iteration.
                c.destroy();
            }
            Timer {
                interval: 30; running: true
                onTriggered: Qt.quit()
            }
        }
    "#);
    qapp.run();

    assert!(backend.borrow().observed.upgrade().is_none(),
        "the engine's deletion must free the value: the proxy held the \
         last strong reference");
}

#[cfg(not(miri))]
fn main() {
    common::run_test!(engine_created_object_is_never_collected);
    common::run_test!(engine_deletion_frees_the_value);
}

#[cfg(miri)]
fn main() {}
