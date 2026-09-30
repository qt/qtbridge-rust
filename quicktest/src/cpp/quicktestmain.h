// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef QUICKTEST_RUST_H
#define  QUICKTEST_RUST_H
#include "rust/cxx.h"

#include <QVariantMap>

namespace rust
{

namespace bridge
{
    int quickTestMain(const rust::Vec<rust::String>& args, const rust::String &name);

    int quickTestMainWithProperties(const rust::Vec<rust::String>& args, const rust::String &name, const QVariantMap &properties);

} // namespace bridge

} // namespace rust

#endif //  QUICKTEST_RUST_H
