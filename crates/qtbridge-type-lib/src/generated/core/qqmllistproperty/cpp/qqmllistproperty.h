// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QQMLLISTPROPERTY_RUST_BRIDGE_H_
#define _QQMLLISTPROPERTY_RUST_BRIDGE_H_

#include <QObject>
#include <QVariant>
#include <QtQml/QQmlListProperty>
#include <cstdint>
#include "qtbridge-type-lib/src/generated/core/qobject/cpp/qobject.h"
#include "rust/cxx.h"

namespace rust::bridge::qqmllistproperty {

QVariant inlineCppFn_list_property_to_qvariant(QMetaType const &meta_type, QObject *object,
                                               uint8_t *data, size_t append_fn, size_t count_fn,
                                               size_t at_fn, size_t clear_fn);

} // namespace rust::bridge::qqmllistproperty

#endif // _QQMLLISTPROPERTY_RUST_BRIDGE_H_
