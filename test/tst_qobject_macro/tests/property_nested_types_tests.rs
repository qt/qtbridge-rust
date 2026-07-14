// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#![cfg(test)]

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use qtbridge::{QObjectHolder, QmlRegister, qobject};
use quicktest::quick_test_main;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
// Number of live `Node` instances: incremented on construction, decremented on
// drop. Must be back to zero once every QML engine has been torn down.
pub static INSTANCE_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[qobject]
pub mod node {
    use super::{Ordering, RefCell, Rc};

    pub struct Node {
        value: i32,
        left: Option<Rc<RefCell<Node>>>,
        right: Option<Rc<RefCell<Node>>>,
    }

    impl Default for Node {
        fn default() -> Self {
            super::INSTANCE_COUNTER.fetch_add(1, Ordering::Relaxed);
            Self {
                value: 0,
                left: None,
                right: None,
            }
        }
    }

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

        pub fn new(value: i32) -> Self {
            let mut node = Self::default();
            node.value = value;
            node
        }

        pub fn set_value(&mut self, value: i32) {
            self.value = value;
            self.value_changed();
        }

        fn get_left(&self) -> Rc<RefCell<Self>> {
            self.left
                .as_ref()
                .expect("Left node is not present")
                .clone()
        }

        fn get_right(&self) -> &Rc<RefCell<Self>> {
            self.right
                .as_ref()
                .expect("Right node is not present")
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
            // Preorder Depth-First Traversal

            let mut result = vec![self.value];
            if let Some (left) = &self.left {
                result.extend(left.borrow().subtree_values());
            }
            if let Some (right) = &self.right {
                result.extend(right.borrow().subtree_values());
            }
            result
        }
    }

    impl Drop for Node {
        fn drop(&mut self) {
            super::INSTANCE_COUNTER.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

#[qobject(Singleton)]
pub mod backend {
    use super::{RefCell, Rc, QObjectHolder};
    use super::node::Node;

    pub struct Backend {
        tree: Rc<RefCell<Node>>,
    }

    impl Backend {
        qproperty!("myTree", Member = tree, Notify = my_tree_changed);

        #[qsignal]
        fn my_tree_changed(&mut self);
    }

    impl Default for Backend {
        fn default() -> Self {
            // Populate following tree
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

            Self {
                tree: nodes[0].clone()
            }
        }
    }
}

pub use node::Node;
pub use backend::Backend;

fn qml_binary_tree_tests() {
    Node::register();
    backend::Backend::register();

    let args = vec![
        file!().into(),
        "-input".into(),
        format!("{MANIFEST_DIR}/tests/qml/binary_tree")
    ];

    let result = quick_test_main(&args, &"binary_tree".into());
    assert_eq!(result, 0, "quick test failed");
    assert_eq!(INSTANCE_COUNTER.load(Ordering::Relaxed), 0, "Node objects are dangling");
}

fn main() {
    #[cfg(not(miri))]
    qml_binary_tree_tests();
}
