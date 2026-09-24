// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QMETATYPE_RUST_BRIDGE_H_
#define _QMETATYPE_RUST_BRIDGE_H_

#include <QMetaType>
#include <QObject>
#include <cstdint>
#include "qtbridge-type-lib/src/core/qmetatypeinterface/cpp/qmetatypeinterface.h"
#include "qtbridge-type-lib/src/core/qobject/cpp/qobject.h"
#include "rust/cxx.h"
#include "rustconv.h"

namespace rust::bridge::qmetatype {

QMetaType QMetaType_Default();
bool QMetaType_Eq(const QMetaType &lhs, const QMetaType &rhs);

QMetaType inlineCppFn_new(int32_t type_id);

QMetaType inlineCppFn_new_with_interface(::QtPrivate::QMetaTypeInterface const *iface);

int32_t inlineCppFn_id(QMetaType const &self);

bool inlineCppFn_is_valid(QMetaType const &self);

rust::String inlineCppFn_name(QMetaType const &self);

void inlineCppFn_register_type(QMetaType const &self);

} // namespace rust::bridge::qmetatype

namespace rust {

template <>
struct IsRelocatable<::QMetaType> : ::std::true_type
{
};

} // namespace rust

#endif // _QMETATYPE_RUST_BRIDGE_H_
