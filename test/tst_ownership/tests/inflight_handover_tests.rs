// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! Registry ownership per crossing kind: slot return values, signal
//! arguments, slot arguments, property reads and object lists. Each test
//! drops the last user handle while (or after) the pointers travel towards
//! QML and asserts that QML observes live objects, or that unobservable
//! objects are reclaimed by `collect_garbage()`.
//!
//! Test for QTBRIDGES-294

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::{QApp, QObjectHolder, QmlRegister, collect_garbage, qobject};

#[derive(Default)]
pub struct Child {}

#[qobject]
impl Child {
    #[qslot(qml_name = "ping")]
    fn ping(&self) -> i32 {
        42
    }
}

pub struct Factory {
    pub child_pings: i32,
    pub child: Rc<RefCell<Child>>,
}

impl Default for Factory {
    fn default() -> Self {
        Self {
            child_pings: 0,
            // A bare `Rc::default()` would have no attached QObject and
            // could not be read as a property.
            child: Child::default_with_attached_qobject(),
        }
    }
}

#[qobject]
impl Factory {
    qproperty!("child", Member = child, Notify = child_changed);

    #[qsignal(qml_name = "childBorn")]
    pub fn child_born(&mut self, child: Rc<RefCell<Child>>);

    #[qsignal(qml_name = "childrenBorn")]
    pub fn children_born(&mut self, children: Vec<Rc<RefCell<Child>>>);

    #[qsignal]
    fn child_changed(&mut self);

    /// Returns a fresh object whose only user handle is the return
    /// value: it drops inside the dispatch, before the engine wraps the
    /// returned pointer, the in-flight case of QTBRIDGES-294.
    #[qslot(qml_name = "makeChild")]
    fn make_child(&mut self) -> Rc<RefCell<Child>> {
        Child::default_with_attached_qobject()
    }

    /// Returns fresh objects whose only user handles are the vector
    /// elements, all dropped when the dispatch returns.
    #[qslot(qml_name = "makeChildren")]
    fn make_children(&mut self) -> Vec<Rc<RefCell<Child>>> {
        (0..3).map(|_| Child::default_with_attached_qobject()).collect()
    }

    /// Emits a fresh object as a signal argument; the only user handle
    /// is the by-value parameter, dropped when the emit returns.
    #[qslot(qml_name = "emitChild")]
    fn emit_child(&mut self) {
        self.child_born(Child::default_with_attached_qobject());
    }

    /// Replaces the `child` property, dropping the last user handle to
    /// the previous object.
    #[qslot(qml_name = "replaceChild")]
    fn replace_child(&mut self) {
        self.child = Child::default_with_attached_qobject();
        self.child_changed();
    }

    #[qslot(qml_name = "adopt")]
    fn adopt(&mut self, child: Rc<RefCell<Child>>) {
        self.child = child;
        self.child_changed();
    }

    /// Calls collect_garbage() while this very dispatch is running and while
    /// the returned object is still in flight.
    #[qslot(qml_name = "makeChildAndCollect")]
    fn make_child_and_collect(&mut self) -> Rc<RefCell<Child>> {
        let child = Child::default_with_attached_qobject();
        // Must spare `child` (the local handle is Rust interest) and
        // must spare `self` (the running dispatch holds a handle).
        qtbridge::collect_garbage();
        child
    }

    #[qslot(qml_name = "reportPing")]
    fn report_ping(&mut self, answer: i32) {
        if answer == 42 {
            self.child_pings += 1;
        }
    }
}

fn run_qml(qml: &str) -> Rc<RefCell<Factory>> {
    let factory = Factory::default_with_attached_qobject();
    let factory_var = factory.borrow().as_qvariant();
    let mut qapp = QApp::new();
    qapp.add_initial_property("factory", &factory_var)
        .load_qml(qml.as_bytes());
    factory
}

fn slot_return_survives_dropping_the_last_rc_in_flight() {
    let factory = run_qml(r#"
        import QtQuick
        Item {
            required property var factory
            Component.onCompleted: {
                let child = factory.makeChild();
                factory.reportPing(child.ping());
            }
        }
    "#);

    assert_eq!(factory.borrow().child_pings, 1,
        "the returned object must arrive alive in QML although Rust dropped \
         its last handle before the engine wrapped the pointer");
}

fn vec_slot_return_survives_dropping_the_last_rcs_in_flight() {
    let factory = run_qml(r#"
        import QtQuick
        Item {
            required property var factory
            Component.onCompleted: {
                let kids = factory.makeChildren();
                for (let i = 0; i < kids.length; ++i) {
                    factory.reportPing(kids[i].ping());
                }
            }
        }
    "#);

    assert_eq!(factory.borrow().child_pings, 3,
        "every element of a returned list must arrive alive in QML although \
         Rust dropped the vector, and with it all handles, in flight");
}

fn signal_argument_survives_dropping_the_last_rc_in_flight() {
    let factory = run_qml(r#"
        import QtQuick
        Item {
            required property var factory
            property var saved
            Component.onCompleted: {
                factory.childBorn.connect(function(c) { saved = c; });
                factory.emitChild();
                // The emitting slot has returned: Rust dropped its last
                // handle to the argument. The saved reference must still be
                // a live object.
                factory.reportPing(saved.ping());
            }
        }
    "#);

    assert_eq!(factory.borrow().child_pings, 1,
        "a saved signal argument must outlive the emit although Rust \
         dropped its last handle when the emit returned");
}

fn property_read_object_survives_rust_replacing_it() {
    let factory = run_qml(r#"
        import QtQuick
        Item {
            required property var factory
            Component.onCompleted: {
                let first = factory.child;
                factory.replaceChild();
                // Rust dropped its last handle to `first` when the property
                // was replaced; the saved reference must still be alive.
                factory.reportPing(first.ping());
            }
        }
    "#);

    assert_eq!(factory.borrow().child_pings, 1,
        "an object read from a property must stay alive for QML although \
         Rust replaced the property and dropped its last handle");
}

fn slot_argument_reacquired_by_rust_stays_owned() {
    let factory = run_qml(r#"
        import QtQuick
        Item {
            required property var factory
            Component.onCompleted: {
                // Rust re-acquires a handle to an object it created and
                // dropped in flight ...
                factory.adopt(factory.makeChild());
                // ... and hands it out again on a later crossing.
                factory.reportPing(factory.child.ping());
            }
        }
    "#);

    assert_eq!(factory.borrow().child_pings, 1,
        "an object handed back into a slot must roundtrip: dropped in \
         flight, re-acquired by Rust, read back out alive");
}

fn unconnected_signal_argument_is_reclaimed_by_collect_garbage() {
    let factory = Factory::default_with_attached_qobject();

    let child = Child::default_with_attached_qobject();
    let weak = Rc::downgrade(&child);

    // No handler is connected: the argument crosses towards QML but is
    // never wrapped by any engine.
    factory.borrow_mut().child_born(child);

    // The registry keeps the object alive through the crossing: a wrapper
    // created later must land on a live object.
    assert!(weak.upgrade().is_some());

    collect_garbage();
    assert!(weak.upgrade().is_none());
}

fn unconnected_vec_signal_arguments_are_reclaimed_by_collect_garbage() {
    let factory = Factory::default_with_attached_qobject();

    let children: Vec<_> = (0..3)
        .map(|_| Child::default_with_attached_qobject())
        .collect();
    let weaks: Vec<_> = children.iter().map(Rc::downgrade).collect();

    factory.borrow_mut().children_born(children);

    assert!(weaks.iter().all(|weak| weak.upgrade().is_some()));

    collect_garbage();
    assert!(weaks.iter().all(|weak| weak.upgrade().is_none()),
        "every element of an unconnected signal's list argument must be \
         reclaimed by collect_garbage()");
}

/// collect_garbage() invoked from inside a running dispatch must spare both the
/// executing object (the dispatch holds a temporary handle) and the object
/// about to be returned (the slot's local handle is Rust interest) — and
/// the returned pointer must still arrive alive in QML afterwards.
fn collect_inside_a_dispatch_spares_running_and_inflight_objects() {
    let factory = run_qml(r#"
        import QtQuick
        Item {
            required property var factory
            Component.onCompleted: {
                let child = factory.makeChildAndCollect();
                factory.reportPing(child.ping());
            }
        }
    "#);

    assert_eq!(factory.borrow().child_pings, 1,
        "a collect_garbage() run mid-dispatch must not free the executing factory \
         or the in-flight child");
}

#[cfg(not(miri))]
fn main() {
    Child::register();

    // Rust hands objects out ...
    slot_return_survives_dropping_the_last_rc_in_flight();
    vec_slot_return_survives_dropping_the_last_rcs_in_flight();
    signal_argument_survives_dropping_the_last_rc_in_flight();
    property_read_object_survives_rust_replacing_it();
    // ... takes them back ...
    slot_argument_reacquired_by_rust_stays_owned();
    // ... survives collection at the worst possible moment ...
    collect_inside_a_dispatch_spares_running_and_inflight_objects();
    // ... and never-arrived ones are reclaimed.
    unconnected_signal_argument_is_reclaimed_by_collect_garbage();
    unconnected_vec_signal_arguments_are_reclaimed_by_collect_garbage();
}

#[cfg(miri)]
fn main() {}
