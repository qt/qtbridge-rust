// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick
import QtTest
import tst_qobject_macro

// Verifies member-based properties work when #[qobject] is on an `impl` block
// (the `BackendImpl` singleton).
Item {
    id: testSuite

    TestCase {
        name: "MemberPropertyImplVarProperty"
        id: test1

        Item {
            id: item1
            property var subtree: BackendImpl.myTree.left
        }
        function test_bind_singleton_property_to_var_property() {
            compare(item1.subtree.subtree_values(), [2, 4])
        }
    }

    TestCase {
        name: "MemberPropertyImplTypedProperty"
        id: test2

        Item {
            id: item2
            property Node subtree: BackendImpl.myTree.right
        }
        function test_bind_singleton_property_to_typed_property() {
            compare(item2.subtree.subtree_values(), [3])
        }
    }
}
