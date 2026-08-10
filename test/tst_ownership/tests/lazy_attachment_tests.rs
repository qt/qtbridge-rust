// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! Lazy attachment: a `#[qobject]` type is a plain Rust type until its
//! first exposure to QML, at which point the bridge attaches its `QObject`
//! and the registry becomes owner of record. User-created
//! `Rc<RefCell<T>>`s need no ceremony, and an object whose `QObject` was
//! deleted heals itself with a fresh one on its next exposure.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use qtbridge::{QApp, QObjectHolder, QmlRegister, collect_garbage, qobject};
use qtbridge::qtbridge_runtime::live_object_count;

thread_local! { static DROP_COUNT: Cell<u32> = Cell::new(0); }

#[qobject]
pub mod child {
    #[derive(Default)]
    pub struct Child {}

    impl Child {
        #[qslot(qml_name = "ping")]
        fn ping(&self) -> i32 {
            42
        }
    }

    impl Drop for Child {
        fn drop(&mut self) {
            super::DROP_COUNT.with(|count| count.set(count.get() + 1));
        }
    }
}
pub use child::Child;

#[qobject]
pub mod backend {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Default)]
    pub struct Backend {
        pub child_pings: i32,
        pub child: Rc<RefCell<crate::Child>>,
    }

    impl Backend {
        qproperty!("child", Member = child, Notify = child_changed);

        #[qsignal]
        pub fn child_changed(&mut self);

        #[qslot(qml_name = "adopt")]
        fn adopt(&mut self, child: Rc<RefCell<crate::Child>>) {
            self.child = child;
            self.child_changed();
        }

        #[qslot(qml_name = "reportPing")]
        fn report_ping(&mut self, answer: i32) {
            if answer == 42 {
                self.child_pings += 1;
            }
        }
    }
}
pub use backend::Backend;

/// Before any exposure, a `#[qobject]` type is a plain Rust value: no
/// QObject, no registry entry, immediate `Drop` on the last handle.
fn unexposed_object_has_plain_rust_lifetime() {
    let baseline = live_object_count();

    let child = Rc::new(RefCell::new(Child::default()));
    assert!(child.borrow().get_qobject_ptr().is_null(),
        "no QObject may exist before the first exposure");
    assert_eq!(live_object_count(), baseline,
        "unexposed objects are not registry-owned");

    DROP_COUNT.with(|count| count.set(0));
    drop(child);
    assert_eq!(DROP_COUNT.with(|count| count.get()), 1,
        "an unexposed object has plain Rc semantics: Drop runs immediately");
}

/// Emitting a signal on a never-exposed object is a no-op, not a panic:
/// without a QObject there are no connections and no receivers.
fn emit_before_exposure_is_a_noop() {
    let backend = Rc::new(RefCell::new(Backend::default()));
    backend.borrow_mut().child_changed();
}

/// An object whose QObject was deleted heals itself: the next exposure
/// attaches a fresh QObject and the registry takes over.
fn object_heals_after_its_qobject_was_deleted() {
    Child::register();
    let backend = Backend::default_with_attached_qobject();
    let property = backend.borrow().as_qvariant();
    let mut qapp = QApp::new();
    qapp.add_initial_property("backend", &property )
        .load_qml(br#"
        import QtQuick
        Item {
            required property var backend
            Component.onCompleted: {
                backend.reportPing(backend.child.ping());
            }
        }
    "#);
    assert_eq!(backend.borrow().child_pings, 1);

    // Stand-in for an engine-owned QObject dying (e.g. with its component):
    // delete the child's QObject explicitly.
    backend.borrow().child.borrow().detach_qobject();
    assert!(backend.borrow().child.borrow().get_qobject_ptr().is_null());

    // The next exposure attaches a fresh QObject; QML sees a live object
    // again instead of a fatal type mismatch.
    let child = backend.borrow().child.clone();
    let child_ptr = Child::rc_ref_cell_to_qobject(&child);
    assert!(!child_ptr.is_null(),
        "re-exposure must attach a fresh QObject");
    drop(child);

    // The healed object is an ordinary registry-owned object again.
    let weak = Rc::downgrade(&backend.borrow().child);
    backend.borrow_mut().child = Rc::new(RefCell::new(Child::default()));
    collect_garbage();
    assert!(weak.upgrade().is_none(),
        "the healed object is collectable like any other");
}

/// The real-world shape of healing: a dynamically created QML object is
/// adopted into a Rust field and its component is destroyed. The engine
/// deletes the QObject; the Rust half survives through the field, and the
/// next property read attaches a fresh QObject instead of aborting.
fn destroyed_qml_component_is_healed_on_next_exposure() {
    Child::register();
    let backend = Backend::default_with_attached_qobject();

    let mut qapp = QApp::new();
    let property = backend.borrow().as_qvariant();
    qapp.add_initial_property("backend", &property)
        .load_qml(br#"
        import QtQuick
        import tst_ownership
        Item {
            id: root
            required property var backend
            Component { id: childComp; Child {} }
            Component.onCompleted: {
                let c = childComp.createObject(root);
                backend.adopt(c);
                // Deferred: the engine deletes the QObject on the next
                // event-loop iteration.
                c.destroy();
            }
            Timer {
                interval: 50; running: true
                onTriggered: {
                    // The adopted object's QObject is gone; this read heals
                    // it with a fresh one.
                    backend.reportPing(backend.child.ping());
                    Qt.quit();
                }
            }
        }
    "#);
    qapp.run();

    assert_eq!(backend.borrow().child_pings, 1,
        "reading the adopted object after its component was destroyed must          heal it, not abort");
    assert!(!backend.borrow().child.borrow().get_qobject_ptr().is_null(),
        "the healed object has a fresh, registry-owned QObject");
}

#[cfg(not(miri))]
fn main() {
    unexposed_object_has_plain_rust_lifetime();
    emit_before_exposure_is_a_noop();
    object_heals_after_its_qobject_was_deleted();
    destroyed_qml_component_is_healed_on_next_exposure();
}

#[cfg(miri)]
fn main() {}
