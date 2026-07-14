// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
//
// Regression test for QTBRIDGES-40 follow-up: Rc<RefCell<T>> member-based properties
// must work in both positions (`mod` and `impl` blocks) of #[qobject] macro.
#![cfg(test)]

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::{QObjectHolder, QmlRegister, qobject};
use quicktest::quick_test_main;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Default)]
pub struct Node {
    value: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

#[qobject]
impl Node {
    qproperty!("value", Member = value, Write = set_value, Notify = value_changed);
    qproperty!("left", Read = get_left, Write = set_left, Notify = left_changed);
    qproperty!("right", Read = get_right, Write = set_right, Notify = right_changed);
    qproperty!("subtreeSum", Read = get_subtree_sum);

    #[qsignal]
    fn value_changed(&mut self);
    #[qsignal]
    fn left_changed(&mut self);
    #[qsignal]
    fn right_changed(&mut self);

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
        self.value_changed();
    }

    fn get_left(&self) -> Rc<RefCell<Self>> {
        self.left.as_ref().expect("Left node is not present").clone()
    }

    fn get_right(&self) -> &Rc<RefCell<Self>> {
        self.right.as_ref().expect("Right node is not present")
    }

    pub fn set_left(&mut self, value: &Rc<RefCell<Self>>) {
        self.left = Some(value.clone());
    }

    pub fn set_right(&mut self, value: Rc<RefCell<Self>>) {
        self.right = Some(value);
    }

    fn get_subtree_sum(&self) -> i32 {
        let mut result = self.value;
        if let Some(left) = &self.left {
            result += left.borrow().get_subtree_sum();
        }
        if let Some(right) = &self.right {
            result += right.borrow().get_subtree_sum();
        }
        result
    }

    #[qslot]
    fn subtree_values(&self) -> Vec<i32> {
        let mut result = vec![self.value];
        if let Some(left) = &self.left {
            result.extend(left.borrow().subtree_values());
        }
        if let Some(right) = &self.right {
            result.extend(right.borrow().subtree_values());
        }
        result
    }
}

fn make_tree() -> Rc<RefCell<Node>> {
    // Populate following tree:
    //     1
    //    / \
    //   2   3
    //  /
    // 4
    let nodes: Vec<_> = (0..4)
        .map(|num| {
            let node = Node::default_with_attached_qobject();
            node.borrow_mut().set_value(num + 1);
            node
        })
        .collect();

    nodes[0].borrow_mut().set_left(&nodes[1]);
    nodes[0].borrow_mut().set_right(nodes[2].clone());
    nodes[1].borrow_mut().set_left(&nodes[3]);

    nodes[0].clone()
}

pub struct Backend {
    tree: Rc<RefCell<Node>>,
}

#[qobject(Singleton)]
impl Backend {
    qproperty!("myTree", Member = tree, Notify = my_tree_changed);

    #[qsignal]
    fn my_tree_changed(&mut self);
}

impl Default for Backend {
    fn default() -> Self {
        Self { tree: make_tree() }
    }
}

pub struct BackendImpl {
    tree: Rc<RefCell<Node>>,
}

#[qobject(Singleton)]
impl BackendImpl {
    qproperty!("myTree", Member = tree, Notify = my_tree_changed);

    #[qsignal]
    fn my_tree_changed(&mut self);
}

impl Default for BackendImpl {
    fn default() -> Self {
        Self { tree: make_tree() }
    }
}

fn qml_member_properties_work_in_mod_and_impl_positions() {
    Backend::register();
    BackendImpl::register();
    Node::register();

    let args = vec![
        file!().into(),
        "-input".into(),
        format!("{MANIFEST_DIR}/tests/qml/member_property_bindings"),
    ];

    let result = quick_test_main(&args, &"member_property".into());
    assert_eq!(result, 0, "quick test failed");
}

fn main() {
    #[cfg(not(miri))]
    qml_member_properties_work_in_mod_and_impl_positions();
}
