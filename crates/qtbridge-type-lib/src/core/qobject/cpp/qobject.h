// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QOBJECT_RUST_BRIDGE_H_
#define _QOBJECT_RUST_BRIDGE_H_

#include <QtCore/QObject>
#include <QtCore/QVariant>
#include "rust/cxx.h"

namespace rust::bridge::qobject {

const QMetaObject *metaObject(const QObject &obj);

void deleteQObject(QObject *obj);

void destructQObject(QObject *obj);

QVariant property(const QObject &obj, rust::Str name);

bool setProperty(QObject *obj, rust::Str name, const QVariant &value);

} // namespace rust::bridge::qobject

#endif // _QOBJECT_RUST_BRIDGE_H_
