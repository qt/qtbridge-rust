// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! End-to-end proof of automatic collection: under a real event loop, the
//! GC-epoch sentinel must reclaim abandoned objects on its own without manually
//! calling `collect_garbage()`. Doubles as a memory test: after creating a pile
//! of garbage, both registry counts must return to their baselines.
//! `gc()` is called in the QmlEngine to simulate the otherwise automatically
//! occuring garbage collector cycle.

#[cfg(not(miri))]
#[path = "../../common/mod.rs"]
mod common;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use qtbridge::{QApp, QmlObject, qobject};
use qtbridge::qtbridge_runtime::registry_owned_count;
use qtbridge::qtbridge_interfaces::attached_object_count;

const GARBAGE: usize = 100;

#[derive(Default)]
pub struct Child {}

#[qobject]
impl Child {
}

#[derive(Default)]
pub struct Backend {
    pub weaks: Vec<Weak<RefCell<Child>>>,
    pub recorded_registry_owned: Option<usize>,
    pub recorded_attached: Option<usize>,
}

#[qobject]
impl Backend {
    /// Creates one piece of garbage: the returned object is discarded by
    /// QML and Rust keeps only a weak handle.
    #[qslot(qml_name = "makeChild")]
    fn make_child(&mut self) -> Rc<RefCell<Child>> {
        let child = Child::default_with_attached_qobject();
        self.weaks.push(Rc::downgrade(&child));
        child
    }

    /// Records the live counts after the sentinel had its turn.
    #[qslot(qml_name = "recordStats")]
    fn record_stats(&mut self) {
        self.recorded_registry_owned = Some(registry_owned_count());
        self.recorded_attached = Some(attached_object_count());
    }
}

fn sentinel_reclaims_garbage_without_manual_collect_garbage() {
    let backend = Backend::default_with_attached_qobject();

    let baseline_registry_owned = registry_owned_count();
    let baseline_attached = attached_object_count();

    // The QML: produce garbage and run a collection. The sentinel's
    // collect_garbage() hands the wrapped garbage to the engine; the second
    // gc() then frees the wrappers and with them the objects. Recording
    // happens one tick later, after the deferred deletions ran.
    let mut qapp = QApp::new();
    qapp.set_initial_object("backend", backend.clone())
        .load_qml(format!(r#"
        import QtQuick
        Item {{
            required property var backend
            Component.onCompleted: {{
                for (let i = 0; i < {GARBAGE}; ++i) {{
                    backend.makeChild();
                }}
                gc();
            }}
            Timer {{
                interval: 30; running: true
                onTriggered: {{
                    gc();
                    statsTimer.start();
                }}
            }}
            Timer {{
                id: statsTimer
                interval: 30
                onTriggered: {{
                    backend.recordStats();
                    Qt.quit();
                }}
            }}
        }}
    "#).as_bytes());
    qapp.run();

    let recorded_registry_owned = backend.borrow().recorded_registry_owned
        .expect("the timer must have fired before quit");
    let recorded_attached = backend.borrow().recorded_attached.unwrap();

    assert_eq!(recorded_registry_owned, baseline_registry_owned,
        "the sentinel must have collected all {GARBAGE} garbage objects \
         without any manual collect_garbage()");
    assert_eq!(recorded_attached, baseline_attached,
        "every garbage proxy pair must be gone");
    assert!(backend.borrow().weaks.iter().all(|weak| weak.upgrade().is_none()),
        "all garbage Rust objects must have been freed");
}

#[cfg(not(miri))]
fn main() {
    common::run_test!(sentinel_reclaims_garbage_without_manual_collect_garbage);
}

#[cfg(miri)]
fn main() {}
