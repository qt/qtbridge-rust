// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QHASH_I32_QBYTEARRAY_RUST_BRIDGE_H_
#define _QHASH_I32_QBYTEARRAY_RUST_BRIDGE_H_

#include <QList>
#include <QHash>
#include <cstdint>
#include "qtbridge-type-lib/src/generated/core/qbytearray/cpp/qbytearray.h"
#include "rust/cxx.h"

using QList_i32 = ::QList<int32_t>;
using QList_QByteArray = ::QList<QByteArray>;
using QHash_i32_QByteArray = ::QHash<int32_t, QByteArray>;

namespace rust::bridge::qhash_i32_qbytearray {

void QHash_Drop(QHash_i32_QByteArray &v);
QHash_i32_QByteArray QHash_Default();
QHash_i32_QByteArray QHash_Clone(const QHash_i32_QByteArray &src);

void inlineCppFn_clear(QHash_i32_QByteArray &self);

bool inlineCppFn_contains(QHash_i32_QByteArray const &self, int32_t const &key);

void inlineCppFn_insert(QHash_i32_QByteArray &self, int32_t const &key, QByteArray const &value);

bool inlineCppFn_is_empty(QHash_i32_QByteArray const &self);

bool inlineCppFn_remove(QHash_i32_QByteArray &self, int32_t const &key);

ptrdiff_t inlineCppFn_size(QHash_i32_QByteArray const &self);

QList_i32 inlineCppFn_keys(QHash_i32_QByteArray const &self);

QList_QByteArray inlineCppFn_values(QHash_i32_QByteArray const &self);

QByteArray inlineCppFn_value(QHash_i32_QByteArray const &self, int32_t const &key);

QByteArray const *inlineCppFn_TraitImpl_std_ops_Index_ref_i32_for_QHash_i32_QByteArray_index(
        QHash_i32_QByteArray const &self, int32_t const &key);

} // namespace rust::bridge::qhash_i32_qbytearray

namespace rust {

template <>
struct IsRelocatable<::QHash_i32_QByteArray> : ::std::true_type
{
};

} // namespace rust

#endif // _QHASH_I32_QBYTEARRAY_RUST_BRIDGE_H_
