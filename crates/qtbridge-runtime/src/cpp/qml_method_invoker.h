// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef QML_METHOD_INVOKER_H
#define QML_METHOD_INVOKER_H

#include "rust/cxx.h"

#include <QObject>
#include <QVariant>
#include <QVariantList>
#include <cstddef>
#include <cstdint>

void on_qobject_destroyed(std::size_t flag_ptr) noexcept; // implemented in rust side via cxx
void connect_destroyed_callback(QObject *obj, std::uintptr_t flag_ptr);

// Invokes `name` on `obj`, resolving it over `obj`'s live meta-object from the
// most-derived class to the base (QML-added members first, then Rust, then the
// C++ base). Among methods with a matching name, picks the first candidate
// whose parameter count matches `args` and whose every argument is convertible
// to the corresponding parameter type, then invokes it by index. Returns false
// if no candidate matches or a conversion fails.
bool invoke_method(QObject *obj, rust::Str name, const QList<QVariant> &args);

#endif // QML_METHOD_INVOKER_H
