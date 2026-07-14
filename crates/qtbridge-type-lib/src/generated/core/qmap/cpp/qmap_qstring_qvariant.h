// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QMAP_QSTRING_QVARIANT_RUST_BRIDGE_H_
#define _QMAP_QSTRING_QVARIANT_RUST_BRIDGE_H_

#include <QList>
#include <QMap>
#include <QVariant>
#include <cstdint>
#include "qtbridge-type-lib/src/generated/core/qstring/cpp/qstring.h"
#include "rust/cxx.h"

using QList_QString = ::QList<QString>;
using QList_QVariant = ::QList<QVariant>;
using QMap_QString_QVariant = ::QMap<QString, QVariant>;

namespace rust::bridge::qmap_qstring_qvariant {

void QMap_Drop(QMap_QString_QVariant &v);
QMap_QString_QVariant QMap_Default();
QMap_QString_QVariant QMap_Clone(const QMap_QString_QVariant &src);

void inlineCppFn_clear(QMap_QString_QVariant &self);

void inlineCppFn_insert(QMap_QString_QVariant &self, QString const &key, QVariant const &value);

bool inlineCppFn_is_empty(QMap_QString_QVariant const &self);

int32_t inlineCppFn_remove(QMap_QString_QVariant &self, QString const &key);

int32_t inlineCppFn_size(QMap_QString_QVariant const &self);

QList_QString inlineCppFn_keys(QMap_QString_QVariant const &self);

QList_QVariant inlineCppFn_values(QMap_QString_QVariant const &self);

QVariant inlineCppFn_value(QMap_QString_QVariant const &self, QString const &key);

QVariant inlineCppFn_TraitImpl_From_ref_QMap_QString_QVariant_for_QVariant_from(
        QMap_QString_QVariant const &value);

bool inlineCppFn_TraitImpl_TryFrom_ref_QVariant_for_QMap_QString_QVariant_try_from(
        QVariant const &from, QMap_QString_QVariant &result);

QVariant const *inlineCppFn_TraitImpl_std_ops_Index_ref_QString_for_QMap_QString_QVariant_index(
        QMap_QString_QVariant const &self, QString const &key);

} // namespace rust::bridge::qmap_qstring_qvariant

namespace rust {

template <>
struct IsRelocatable<::QMap_QString_QVariant> : ::std::true_type
{
};

} // namespace rust

#endif // _QMAP_QSTRING_QVARIANT_RUST_BRIDGE_H_
