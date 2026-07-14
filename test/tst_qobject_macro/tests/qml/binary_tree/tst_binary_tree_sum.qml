// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtTest
import tst_qobject_macro

TestCase {
    name: "BinaryTreeSum"

    Item {
        id: testTree

        // Define a following tree
        //     6
        //    / \
        //   4   5
        //  / \   \
        // 1   2   3

        Node {
            id: node1
            value: 1
        }
        Node {
            id: node2
            value: 2
        }
        Node {
            id: node3
            value: 3
        }
        Node {
            id: node4
            value: 4
            left: node1
            right: node2
        }
        Node {
            id: node5
            value: 5
            right: node3
        }
        Node {
            id: root
            value: 6
            left: node4
            right: node5
        }
    }

    function test_binary_tree_sum() {
        compare(root.subtreeSum, 21)
    }
}
