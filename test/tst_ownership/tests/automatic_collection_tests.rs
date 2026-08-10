// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! End-to-end proof of automatic collection: under a real event loop, the
//! GC-epoch sentinel must reclaim abandoned objects on its own without manually
//! calling `collect_garbage()`. Doubles as a memory test: after creating a pile
//! of garbage, the registry and proxy counts must return to their baselines.
//! `gc()` is called in the QmlEngine to simulate the otherwise automatically
//! occuring garbage collector cycle.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use qtbridge::{QApp, QObjectHolder, qobject};
use qtbridge::qtbridge_runtime::live_object_count;
use qtbridge::qtbridge_interfaces::live_proxy_count;

const GARBAGE: usize = 100;

#[derive(Default)]
pub struct Child {}

#[qobject]
impl Child {
}

#[derive(Default)]
pub struct Backend {
    pub weaks: Vec<Weak<RefCell<Child>>>,
    pub recorded_objects: Option<usize>,
    pub recorded_proxies: Option<usize>,
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
        self.recorded_objects = Some(live_object_count());
        self.recorded_proxies = Some(live_proxy_count());
    }
}

fn sentinel_reclaims_garbage_without_manual_collect_garbage() {
    let backend = Backend::default_with_attached_qobject();
    let backend_var = backend.borrow().as_qvariant();

    let baseline_objects = live_object_count();
    let baseline_proxies = live_proxy_count();

    // The QML: produce garbage and run a collection. The sentinel's
    // collect_garbage() hands the wrapped garbage to the engine; the second
    // gc() then frees the wrappers and with them the objects. Recording
    // happens one tick later, after the deferred deletions ran.
    let mut qapp = QApp::new();
    qapp.add_initial_property("backend", &backend_var)
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

    let recorded_objects = backend.borrow().recorded_objects
        .expect("the timer must have fired before quit");
    let recorded_proxies = backend.borrow().recorded_proxies.unwrap();

    assert_eq!(recorded_objects, baseline_objects,
        "the sentinel must have collected all {GARBAGE} garbage objects \
         without any manual collect_garbage()");
    assert_eq!(recorded_proxies, baseline_proxies,
        "every garbage proxy pair must be gone");
    assert!(backend.borrow().weaks.iter().all(|weak| weak.upgrade().is_none()),
        "all garbage Rust objects must have been freed");
}

#[cfg(not(miri))]
fn main() {
    sentinel_reclaims_garbage_without_manual_collect_garbage();
}

#[cfg(miri)]
fn main() {}
