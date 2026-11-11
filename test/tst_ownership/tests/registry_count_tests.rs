// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#[cfg(not(miri))]
#[path = "../../common/mod.rs"]
mod common;

use qtbridge::qtbridge_runtime::{live_object_count, live_proxy_count};
use qtbridge::{QApp, QmlElement, QmlObject, qobject};

#[derive(Default)]
struct TestObject {}

#[qobject]
impl TestObject {}

fn rust_owned_object_increases_both_counts() {
    let owned_count_before = live_object_count();
    let proxies_count_before = live_proxy_count();

    let object = TestObject::default_with_attached_qobject();
    assert_eq!(live_object_count(), owned_count_before + 1);
    assert_eq!(live_proxy_count(), proxies_count_before + 1);

    drop(object);
    qtbridge::collect_garbage();
    assert_eq!(live_object_count(), owned_count_before);
    assert_eq!(live_proxy_count(), proxies_count_before);
}
fn engine_owned_object_increases_only_proxy_count() {
    TestObject::register();
    let owned_before = live_object_count();
    let proxies_before = live_proxy_count();

    {
        let mut qapp = QApp::new();
        qapp.load_qml(
            br#"
            import tst_ownership
            TestObject {}
        "#,
        );
        assert_eq!(live_object_count(), owned_before);
        assert_eq!(live_proxy_count(), proxies_before + 1);
    }
    assert_eq!(live_object_count(), owned_before);
    assert_eq!(live_proxy_count(), proxies_before);
}

#[cfg(not(miri))]
fn main() {
    common::run_test!(rust_owned_object_increases_both_counts);
    common::run_test!(engine_owned_object_increases_only_proxy_count);
}

#[cfg(miri)]
fn main() {}
