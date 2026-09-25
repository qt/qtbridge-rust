// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QMETATYPE_RUST_BRIDGE_H_
#define _QMETATYPE_RUST_BRIDGE_H_

#include <QMetaType>
#include <cstdint>
#include "qtbridge-type-lib/src/core/qmetatypeinterface/cpp/qmetatypeinterface.h"
#include "rust/cxx.h"

namespace rust::bridge::qmetatype {

QMetaType defaultQMetaType();

QMetaType newWithInterface(::QtPrivate::QMetaTypeInterface const &iface);

bool eq(QMetaType const &lhs, QMetaType const &rhs);

int32_t id(QMetaType const &obj);

rust::String name(QMetaType const &obj);

void registerType(QMetaType const &obj);

} // namespace rust::bridge::qmetatype

namespace rust {

template <>
struct IsRelocatable<::QMetaType> : ::std::true_type
{
};

} // namespace rust

#endif // _QMETATYPE_RUST_BRIDGE_H_
