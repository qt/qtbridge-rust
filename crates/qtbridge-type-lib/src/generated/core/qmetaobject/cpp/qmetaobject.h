// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QMETAOBJECT_RUST_BRIDGE_H_
#define _QMETAOBJECT_RUST_BRIDGE_H_

#include <QList>
#include <QMetaObject>
#include <QMetaType>
#include <QVariant>
#include "rust/cxx.h"

namespace rust::bridge::qmetaobject {

QMetaType inlineCppFn_meta_type(QMetaObject const &self);

bool inlineCppFn_inherits(QMetaObject const &self, QMetaObject const &base);

} // namespace rust::bridge::qmetaobject

#endif // _QMETAOBJECT_RUST_BRIDGE_H_
