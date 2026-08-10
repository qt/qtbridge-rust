// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! Lifetime of objects QML has seen: QML references keep them alive across
//! `collect_garbage()`, the garbage collector only ever reclaims their JS wrappers
//! (never the objects, they keep `CppOwnership` for life, even across
//! engine destruction), and abandoned objects are reclaimed by a garbage
//! collection followed by `collect_garbage()`.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use qtbridge::{QApp, QObjectHolder, QPropertyMember, QmlRegister, collect_garbage, qobject};
use qtbridge::qtbridge_type_lib::{
    QGuiApplication, QQmlApplicationEngine, QSignalSpy, QString, QVariantMap,
};

#[derive(Default)]
pub struct TestObject {
}

#[qobject]
impl TestObject {
}

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
    pub child: Rc<RefCell<Child>>,
    pub last_made: Weak<RefCell<Child>>,
}

#[qobject]
impl Backend {
    qproperty!("child", Member = child, Notify = child_changed);

    #[qsignal]
    fn child_changed(&mut self);

    #[qslot(qml_name = "makeChild")]
    fn make_child(&mut self) -> Rc<RefCell<Child>> {
        let child: Rc<RefCell<Child>> = Default::default();
        self.last_made = Rc::downgrade(&child);
        child
    }

    #[qslot(qml_name = "reportPing")]
    fn report_ping(&mut self, answer: i32) {
        if answer == 42 {
            self.child_pings += 1;
        }
    }
}


fn qml_reference_keeps_object_alive_after_last_rc_drop() {
    let obj = TestObject::default_with_attached_qobject();
    let weak = Rc::downgrade(&obj);
    let obj_var = obj.borrow().as_qvariant();

    let mut qapp = QApp::new();
    qapp.add_initial_property("testObject", &obj_var)
        .load_qml(br#"
        import QtQuick
        import tst_ownership
        Item {
            required property TestObject testObject;
        }
    "#);

    // The QML property still references the object: even after Rust drops
    // its last handle and a collection runs, the registry keeps it.
    drop(obj);
    collect_garbage();
    assert!(weak.upgrade().is_some(),
        "the live JS wrapper must keep the object alive across collect_garbage()");
}

/// The garbage collector can only reclaim JS wrappers, never the objects:
/// with `CppOwnership` for life, a `gc()` with no QML references leaves the
/// object fully usable and QML simply re-wraps it on the next access.
fn object_survives_gc_while_rust_holds_a_handle() {
    let backend = Backend::default_with_attached_qobject();
    let backend_var = backend.borrow().as_qvariant();

    let mut qapp = QApp::new();
    qapp.add_initial_property("backend", &backend_var)
        .load_qml(br#"
        import QtQuick
        Item {
            required property var backend
            Component.onCompleted: {
                let a = backend.child;
                a = null;
                gc();
                // The wrapper may be gone; the object is not. Re-reading
                // the property wraps it again.
                backend.reportPing(backend.child.ping());
            }
        }
    "#);

    assert_eq!(backend.borrow().child_pings, 1,
        "an object must survive garbage collection while Rust holds a \
         handle, although QML dropped all references");
}

/// Once Rust holds no handle, `collect_garbage()` hands the object to the
/// engine and the next collection of its wrapper reclaims it.
fn released_object_is_reclaimed_by_handover_plus_gc() {
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
                let c = backend.makeChild();
                backend.reportPing(c.ping());
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

    assert_eq!(backend.borrow().child_pings, 1);
    // Hands the abandoned object to the engine; the gc() in the timer
    // frees its wrapper and the event loop runs the deferred deletion.
    collect_garbage();
    qapp.run();
    assert!(backend.borrow().last_made.upgrade().is_none(),
        "handover plus gc() must reclaim the object once QML let go");
}

/// An object whose pointer went towards QML but was never wrapped is
/// reclaimed by `collect_garbage()` alone once Rust drops its handles.
fn unreferenced_object_is_reclaimed_by_collect_garbage() {
    let obj = TestObject::default_with_attached_qobject();
    let weak = Rc::downgrade(&obj);
    // The pointer travels towards QML, but nothing ever wraps it: The
    // situation of a signal emitted without a connected handler.
    let _var = obj.borrow().as_qvariant();

    drop(obj);
    // Still owned: a wrapper created later must land on a live object.
    assert!(weak.upgrade().is_some());

    collect_garbage();
    assert!(weak.upgrade().is_none());
}

/// QObjects keep `CppOwnership` for life: they survive the destruction of
/// the engine that wrapped them and remain fully usable from Rust.
fn object_survives_engine_death() {
    let _app = QGuiApplication::new();
    let obj = TestObject::default_with_attached_qobject();
    let weak = Rc::downgrade(&obj);
    let spy = QSignalSpy::new(
        unsafe { &*obj.borrow().get_qobject_ptr() },
        "destroyed"
    );

    {
        let mut engine = QQmlApplicationEngine::new();
        let mut props = QVariantMap::default();
        props.insert(QString::from("testObject"), obj.borrow().as_qvariant());
        engine.pin_mut().set_initial_properties(&props);
        engine.pin_mut().load_data(&r#"
            import QtQuick
            Item {
                required property var testObject;
            }
        "#.into(), &Default::default());
    }
    // The engine is gone; the object and its QObject are not.
    assert_eq!(spy.count(), 0);
    assert!(!obj.borrow().get_qobject_ptr().is_null());

    // From here on it is an ordinary registry-owned object again.
    drop(obj);
    collect_garbage();
    assert_eq!(spy.count(), 1);
    assert!(weak.upgrade().is_none());
}

fn as_qvariant_roundtrip_preserves_identity() {
    let obj = TestObject::default_with_attached_qobject();
    let var = obj.borrow().as_qvariant();

    // Exercises the checked downcast from the erased allocation back to the
    // typed handle.
    let roundtripped: Rc<std::cell::RefCell<TestObject>> =
        QPropertyMember::from_qvariant(&var).expect("QVariant holds a TestObject");
    assert!(Rc::ptr_eq(&obj, &roundtripped));
}

#[cfg(not(miri))]
fn main() {
    Child::register();
    TestObject::register();
    qml_reference_keeps_object_alive_after_last_rc_drop();
    object_survives_gc_while_rust_holds_a_handle();
    released_object_is_reclaimed_by_handover_plus_gc();
    unreferenced_object_is_reclaimed_by_collect_garbage();
    object_survives_engine_death();
    as_qvariant_roundtrip_preserves_identity();
}

#[cfg(miri)]
fn main() {}
