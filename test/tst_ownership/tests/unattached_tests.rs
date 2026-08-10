// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

//! Behavior of `#[qobject]` instances without an attached `QObject`:
//! they are plain Rust values and must not panic when used as such.

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::qobject;

#[qobject]
pub mod backend {
    #[derive(Default)]
    pub struct Backend {}

    impl Backend {
        #[qsignal]
        pub fn plain_signal(&mut self);

        #[qsignal]
        pub fn valued_signal(&mut self, value: i32);
    }
}
pub use backend::Backend;

/// Emitting a signal without an attached QObject is a no-op, not a panic:
/// no QObject means no connections and no receivers.
fn emit_without_attached_qobject_is_a_noop() {
    let backend = Rc::new(RefCell::new(Backend::default()));
    backend.borrow_mut().plain_signal();
    backend.borrow_mut().valued_signal(42);
}

#[cfg(not(miri))]
fn main() {
    emit_without_attached_qobject_is_a_noop();
}

#[cfg(miri)]
fn main() {}
