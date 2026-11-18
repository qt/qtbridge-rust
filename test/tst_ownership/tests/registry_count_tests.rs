// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#[cfg(not(miri))]
#[path = "../../common/mod.rs"]
mod common;

use qtbridge::qtbridge_runtime::{registry_owned_count, attached_object_count};
use qtbridge::{QApp, QmlElement, QmlObject, qobject};

#[derive(Default)]
struct TestObject {}

#[qobject]
impl TestObject {}

fn rust_owned_object_increases_both_counts() {
    let registry_owned_before = registry_owned_count();
    let attached_before = attached_object_count();

    let object = TestObject::default_with_attached_qobject();
    assert_eq!(registry_owned_count(), registry_owned_before + 1);
    assert_eq!(attached_object_count(), attached_before + 1);

    drop(object);
    qtbridge::collect_garbage();
    assert_eq!(registry_owned_count(), registry_owned_before);
    assert_eq!(attached_object_count(), attached_before);
}
fn engine_owned_object_increases_only_attached_count() {
    TestObject::register();
    let registry_owned_before = registry_owned_count();
    let attached_before = attached_object_count();

    {
        let mut qapp = QApp::new();
        qapp.load_qml(
            br#"
            import tst_ownership
            TestObject {}
        "#,
        );
        assert_eq!(registry_owned_count(), registry_owned_before);
        assert_eq!(attached_object_count(), attached_before + 1);
    }
    assert_eq!(registry_owned_count(), registry_owned_before);
    assert_eq!(attached_object_count(), attached_before);
}

#[cfg(not(miri))]
fn main() {
    common::run_test!(rust_owned_object_increases_both_counts);
    common::run_test!(engine_owned_object_increases_only_attached_count);
}

#[cfg(miri)]
fn main() {}
