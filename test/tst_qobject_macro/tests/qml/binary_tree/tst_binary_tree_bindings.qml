// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtTest
import tst_qobject_macro

Item {
    id: testSuite

    TestCase {
        name: "BinaryTreeBindingsVarProperty"
        id: test1

        Item {
            id: item1
            property var subtree: Backend.myTree.left
        }
        function test_bind_singleton_property_to_var_property() {
            compare(item1.subtree.subtree_values(), [2, 4])
        }
    }

    TestCase {
        name: "BinaryTreeBindingsTypedProperty"
        id: test2

        Item {
            id: item2
            property Node subtree: Backend.myTree.right
        }
        function test_bind_singleton_property_to_typed_property() {
            compare(item2.subtree.subtree_values(), [3])
        }
    }
}
