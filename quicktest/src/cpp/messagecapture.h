// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef QUICKTEST_MESSAGECAPTURE_RUST_H
#define QUICKTEST_MESSAGECAPTURE_RUST_H
#include "rust/cxx.h"

namespace rust
{

namespace bridge
{
    // Installs a Qt message handler that records emitted messages
    // (qDebug/qWarning/QML diagnostics) and forwards them to the
    // previously installed handler.
    void installMessageCapture();

    // Returns the messages captured since the last call and clears the buffer.
    rust::Vec<rust::String> takeCapturedMessages();

} // namespace bridge

} // namespace rust

#endif // QUICKTEST_MESSAGECAPTURE_RUST_H
