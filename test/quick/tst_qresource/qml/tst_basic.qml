// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

import QtQuick 2.3
import QtTest 1.0

TestCase {
    name: "test_with_text"
    id: test

    TextEdit {
        id: text
    }

    function test_samefolder_noprefix() {
        text.clear()
        compare(text.text, "")
        text.textDocument.modified = false
        text.textDocument.source = "qrc:/example.txt"
        compare(text.text, "Hello QML!\n")
    }

    function test_subfolder_noprefix() {
        text.clear()
        compare(text.text, "")
        text.textDocument.modified = false
        text.textDocument.source = "qrc:/text/example.txt"
        compare(text.text, "Hello Rust!\n")
    }

    function test_samefolder() {
        text.clear()
        compare(text.text, "")
        text.textDocument.modified = false
        text.textDocument.source = "qrc:/samefolder/example.txt"
        compare(text.text, "Hello QML!\n")
    }

    function test_subfolder() {
        text.clear()
        compare(text.text, "")
        text.textDocument.modified = false
        text.textDocument.source = "qrc:/subfolder/text/example.txt"
        compare(text.text, "Hello Rust!\n")
    }

    function test_samefolder_extra_prefix() {
        text.clear()
        compare(text.text, "")
        text.textDocument.modified = false
        text.textDocument.source = "qrc:/samefolder/addedfolder/example.txt"
        compare(text.text, "Hello QML!\n")
    }

    function test_subfolder_extra_prefix() {
        text.clear()
        compare(text.text, "")
        text.textDocument.modified = false
        text.textDocument.source = "qrc:/subfolder/addedfolder/text/example.txt"
        compare(text.text, "Hello Rust!\n")
    }

    function test_dir_noprefix() {
        text.clear()
        compare(text.text, "")
        text.textDocument.modified = false
        text.textDocument.source = "qrc:/dir/example.txt"
        compare(text.text, "Hello Rust from dir!\n")
    }

    function test_dir_noprefix_nested() {
        text.clear()
        compare(text.text, "")
        text.textDocument.modified = false
        text.textDocument.source = "qrc:/dir/sub_dir/example.txt"
        compare(text.text, "Hello Rust from sub_dir!\n")
    }

    function test_dir_prefix() {
        text.clear()
        compare(text.text, "")
        text.textDocument.modified = false
        text.textDocument.source = "qrc:/dirprefix/dir/example.txt"
        compare(text.text, "Hello Rust from dir!\n")
    }

    function test_dir_prefix_nested() {
        text.clear()
        compare(text.text, "")
        text.textDocument.modified = false
        text.textDocument.source = "qrc:/dirprefix/dir/sub_dir/example.txt"
        compare(text.text, "Hello Rust from sub_dir!\n")
    }
}
