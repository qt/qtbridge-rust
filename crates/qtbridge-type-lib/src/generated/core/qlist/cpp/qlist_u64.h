// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLIST_U64_RUST_BRIDGE_H_
#define _QLIST_U64_RUST_BRIDGE_H_

#include <QList>
#include <cstdint>
#include "rust/cxx.h"

using QList_u64 = ::QList<uint64_t>;

namespace rust::bridge::qlist_u64 {

void QList_Drop(QList_u64 &v);
QList_u64 QList_Default();
QList_u64 QList_Clone(const QList_u64 &src);
bool QList_Eq(const QList_u64 &lhs, const QList_u64 &rhs);

void inlineCppFn_append(QList_u64 &self, uint64_t value);

void inlineCppFn_clear(QList_u64 &self);

bool inlineCppFn_contains(QList_u64 const &self, uint64_t const &value);

void inlineCppFn_push_back(QList_u64 &self, uint64_t value);

void inlineCppFn_remove(QList_u64 &self, ptrdiff_t i, ptrdiff_t n);

void inlineCppFn_reserve(QList_u64 &self, size_t size);

ptrdiff_t inlineCppFn_size(QList_u64 const &self);

uint64_t const &inlineCppFn_first(QList_u64 const &self);

uint64_t const &inlineCppFn_last(QList_u64 const &self);

rust::Vec<uint64_t> inlineCppFn_TraitImpl_From_ref_QList_u64_for_Vec_u64_from(QList_u64 const &src);

uint64_t const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_u64_index(QList_u64 const &self,
                                                                              size_t index);

bool inlineCppFn_TraitImpl_PartialEq_array_of_u64_N_for_QList_u64_eq(
        QList_u64 const &self, rust::Slice<uint64_t const> rhs);

} // namespace rust::bridge::qlist_u64

namespace rust {

template <>
struct IsRelocatable<::QList_u64> : ::std::true_type
{
};

} // namespace rust

#endif // _QLIST_U64_RUST_BRIDGE_H_
