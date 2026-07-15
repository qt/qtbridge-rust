// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLIST_I64_RUST_BRIDGE_H_
#define _QLIST_I64_RUST_BRIDGE_H_

#include <QList>
#include <cstdint>
#include "rust/cxx.h"

using QList_i64 = ::QList<int64_t>;

namespace rust::bridge::qlist_i64 {

void QList_Drop(QList_i64 &v);
QList_i64 QList_Default();
QList_i64 QList_Clone(const QList_i64 &src);
bool QList_Eq(const QList_i64 &lhs, const QList_i64 &rhs);

void inlineCppFn_append(QList_i64 &self, int64_t value);

void inlineCppFn_clear(QList_i64 &self);

bool inlineCppFn_contains(QList_i64 const &self, int64_t const &value);

void inlineCppFn_push_back(QList_i64 &self, int64_t value);

void inlineCppFn_remove(QList_i64 &self, ptrdiff_t i, ptrdiff_t n);

void inlineCppFn_reserve(QList_i64 &self, size_t size);

ptrdiff_t inlineCppFn_size(QList_i64 const &self);

int64_t const &inlineCppFn_first(QList_i64 const &self);

int64_t const &inlineCppFn_last(QList_i64 const &self);

rust::Vec<int64_t> inlineCppFn_TraitImpl_From_ref_QList_i64_for_Vec_i64_from(QList_i64 const &src);

int64_t const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_i64_index(QList_i64 const &self,
                                                                             size_t index);

bool inlineCppFn_TraitImpl_PartialEq_array_of_i64_N_for_QList_i64_eq(
        QList_i64 const &self, rust::Slice<int64_t const> rhs);

} // namespace rust::bridge::qlist_i64

namespace rust {

template <>
struct IsRelocatable<::QList_i64> : ::std::true_type
{
};

} // namespace rust

#endif // _QLIST_I64_RUST_BRIDGE_H_
