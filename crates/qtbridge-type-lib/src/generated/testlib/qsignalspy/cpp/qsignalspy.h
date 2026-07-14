// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QSIGNALSPY_RUST_BRIDGE_H_
#define _QSIGNALSPY_RUST_BRIDGE_H_

#include <QSignalSpy>
#include <QList>
#include <QVariant>
#include <cstdint>
#include <memory>
#include "qtbridge-type-lib/src/generated/core/qobject/cpp/qobject.h"
#include "rust/cxx.h"
#include "rustconv.h"

namespace rust::bridge::qsignalspy {

std::unique_ptr<QSignalSpy> inlineCppFn_new(QObject const &qobject, rust::Str signal_name);

ptrdiff_t inlineCppFn_count(QSignalSpy const &self);

QVariantList inlineCppFn_take_at(QSignalSpy &self, ptrdiff_t idx);

QVariantList inlineCppFn_take_first(QSignalSpy &self);

QVariantList inlineCppFn_take_last(QSignalSpy &self);

} // namespace rust::bridge::qsignalspy

#endif // _QSIGNALSPY_RUST_BRIDGE_H_
