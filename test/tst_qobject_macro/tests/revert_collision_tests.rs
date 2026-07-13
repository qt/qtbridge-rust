// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]

// Exploratory: what happens when a Rust #[qslot] collides with a slot inherited
// from the base model class (QAbstractItemModel::revert(), a benign public
// slot)? The duplicate-name check only looks at Rust-declared members, not
// inherited base-class methods, so these compile and register.

use std::collections::HashMap;
use qtbridge_type_lib::QGuiApplication;
use qtbridge::{qobject, QObjectHolder, QModelItem};
use qtbridge::invoke_method;

#[derive(Clone, Debug, Default, QModelItem)]
pub struct Item {
    display: String,
}

// Case A: Rust slot with the SAME signature as the inherited revert().
#[qobject(Base = QListModel)]
mod same_sig {
    use qtbridge::QListModel;
    #[derive(Default)]
    pub struct RevertSame {
        pub ran: bool,
        list: Vec<crate::Item>,
    }

    impl RevertSame {
        #[qslot(qml_name = "revert")]
        fn shadow_revert(&mut self) {
            self.ran = true;
        }
    }

    impl QListModel for RevertSame {
        type Item = crate::Item;
        fn len(&self) -> usize { self.list.len() }
        fn get(&self, i: usize) -> Option<&Self::Item> { self.list.get(i) }
    }
}
pub use same_sig::RevertSame;

// Case B: Rust slot that OVERLOADS revert() with an int parameter.
#[qobject(Base = QListModel)]
mod with_arg {
    use qtbridge::QListModel;

    #[derive(Default)]
    pub struct RevertArg {
        pub received: i32,
        list: Vec<crate::Item>,
    }

    impl RevertArg {
        #[qslot(qml_name = "revert")]
        fn shadow_revert_int(&mut self, value: i32) {
            self.received = value;
        }
    }

    impl QListModel for RevertArg {
        type Item = crate::Item;
        fn len(&self) -> usize { self.list.len() }
        fn get(&self, i: usize) -> Option<&Self::Item> { self.list.get(i) }
    }
}
pub use with_arg::RevertArg;

#[cfg(not(miri))]
fn main() {
    let app = QGuiApplication::new();

    // Case A: the Rust revert() shares its name with the inherited
    // QAbstractItemModel::revert(). Resolution is most-derived-first, so the
    // Rust slot (more derived) is invoked, not the base.
    let a = RevertSame::default_with_attached_qobject();
    a.borrow().get_qml_method_invoker().invoke_method("revert");
    app.process_events();
    app.process_events();
    assert!(a.borrow().ran,
        "expected the Rust revert() to run, but it did not (base revert() ran instead)");

    // Case B: the Rust revert(int) shares its name with the inherited zero-arg
    // revert(). The candidate must match name AND argument count, so the int
    // argument selects the Rust slot rather than dropping into the base.
    let b = RevertArg::default_with_attached_qobject();
    let inv = b.borrow().get_qml_method_invoker();
    invoke_method!(inv, "revert", 42);
    app.process_events();
    app.process_events();
    assert_eq!(b.borrow().received, 42,
        "expected the Rust revert(int) to receive 42; 0 means the base revert() ran and dropped the argument");
}

#[cfg(miri)]
fn main() {}
