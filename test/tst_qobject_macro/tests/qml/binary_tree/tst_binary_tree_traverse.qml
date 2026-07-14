// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtTest
import tst_qobject_macro

TestCase {
    name: "BinaryTreeTraverse"

    Item {
        id: testTree

        // Define a following tree
        //     7
        //    / \
        //   5   6
        //  / \ / \
        // 1  2 3  4

        Node {
            id: root
            value: 7
            left: Node {
                value: 5
                left: Node {
                    value: 1
                }
                right: Node {
                    value: 2
                }
            }
            right: Node {
                value: 6
                left: Node {
                    value: 3
                }
                right: Node {
                    value: 4
                }
            }
        }
    }

    function test_node_values() {
        compare(root.subtree_values(), [7, 5, 1, 2, 6, 3, 4])
    }
}
