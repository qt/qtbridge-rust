// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtTest
import tst_qobject_macro

// Verifies member-based properties work when #[qobject] is on a `mod` block
// (the `Backend` singleton).
Item {
    id: testSuite

    TestCase {
        name: "MemberPropertyModVarProperty"
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
        name: "MemberPropertyModTypedProperty"
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
