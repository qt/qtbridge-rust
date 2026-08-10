// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! Objects returned from slot invocations must keep `CppOwnership`: the
//! engine flips implicitly-owned Q_INVOKABLE return values to
//! `JavaScriptOwnership` and would delete them at garbage collection even
//! while Rust holds handles. We set `CppOwnership` explicitly so the
//! objects should survive.

use qtbridge::{QApp, QObjectHolder, qobject};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Child {}

#[qobject]
impl Child {
}

#[derive(Default)]
pub struct Backend {
    pub child: Option<Rc<RefCell<crate::Child>>>,
}

#[qobject]
impl Backend {
    #[qslot(qml_name = "makeChild")]
    fn make_child(&mut self) -> Rc<RefCell<crate::Child>> {
        let child = crate::Child::default_with_attached_qobject();
        self.child = Some(child.clone());
        child
    }

    /// Returns a fresh object without keeping a handle: the only Rust
    /// interest is the in-flight return value.
    #[qslot(qml_name = "abandonChild")]
    fn abandon_child(&mut self) -> Rc<RefCell<crate::Child>> {
        crate::Child::default_with_attached_qobject()
    }

    #[qslot(qml_name = "adopt")]
    fn adopt(&mut self, child: Rc<RefCell<crate::Child>>) {
        self.child = Some(child);
    }

    #[qslot(qml_name = "collectGarbage")]
    fn collect_garbage(&mut self) {
        qtbridge::collect_garbage();
    }
}

fn slot_returned_object_survives_gc_while_rust_holds_it() {
    let backend = Backend::default_with_attached_qobject();
    let backend_var = backend.borrow().as_qvariant();

    let mut qapp = QApp::new();
    qapp.add_initial_property("backend", &backend_var)
        .load_qml(br#"
        import QtQuick
        Item {
            required property var backend
            Component.onCompleted: {
                // The returned wrapper is dropped immediately; the next
                // garbage collection frees it. If there is not CppOwnership
                // set correctly, the underlying object will be deleted.
                //
                backend.makeChild();
                gc();
            }
            Timer {
                interval: 50; running: true
                onTriggered: Qt.quit()
            }
        }
    "#);
    qapp.run();

    let child = backend.borrow().child.clone()
        .expect("the slot must have run");
    assert!(!child.borrow().get_qobject_ptr().is_null(),
        "the engine must not delete a slot-returned object that Rust still references");
}

/// A handed-over object that re-enters Rust must be pinned back to
/// `CppOwnership`: the wrapper's collection must no longer delete it.
fn handed_over_object_reacquired_by_rust_is_repinned() {
    let backend = Backend::default_with_attached_qobject();
    let backend_var = backend.borrow().as_qvariant();

    let mut qapp = QApp::new();
    qapp.add_initial_property("backend", &backend_var)
        .load_qml(br#"
        import QtQuick
        Item {
            required property var backend
            Component.onCompleted: {
                // Rust drops its only handle in flight; QML is the sole user.
                let c = backend.abandonChild();
                // No Rust interest, live wrapper: hands the object to the
                // engine.
                backend.collectGarbage();
                // The object re-enters Rust, which takes ownership back.
                backend.adopt(c);
                c = null;
            }
            Timer {
                interval: 30; running: true
                onTriggered: {
                    gc();
                    quitTimer.start();
                }
            }
            Timer {
                id: quitTimer
                interval: 30
                onTriggered: Qt.quit()
            }
        }
    "#);
    qapp.run();

    let child = backend.borrow().child.clone()
        .expect("adopt must have run");
    assert!(!child.borrow().get_qobject_ptr().is_null(),
        "the engine must not delete a handed-over object after Rust \
         re-acquired a handle to it");
}

#[cfg(not(miri))]
fn main() {
    slot_returned_object_survives_gc_while_rust_holds_it();
    handed_over_object_reacquired_by_rust_is_repinned();
}

#[cfg(miri)]
fn main() {}
