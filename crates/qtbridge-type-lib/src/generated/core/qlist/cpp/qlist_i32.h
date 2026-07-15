// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLIST_I32_RUST_BRIDGE_H_
#define _QLIST_I32_RUST_BRIDGE_H_

#include <QList>
#include <cstdint>
#include "rust/cxx.h"

using QList_i32 = ::QList<int32_t>;

namespace rust::bridge::qlist_i32 {

void QList_Drop(QList_i32 &v);
QList_i32 QList_Default();
QList_i32 QList_Clone(const QList_i32 &src);
bool QList_Eq(const QList_i32 &lhs, const QList_i32 &rhs);

void inlineCppFn_append(QList_i32 &self, int32_t value);

void inlineCppFn_clear(QList_i32 &self);

bool inlineCppFn_contains(QList_i32 const &self, int32_t const &value);

void inlineCppFn_push_back(QList_i32 &self, int32_t value);

void inlineCppFn_remove(QList_i32 &self, ptrdiff_t i, ptrdiff_t n);

void inlineCppFn_reserve(QList_i32 &self, size_t size);

ptrdiff_t inlineCppFn_size(QList_i32 const &self);

int32_t const &inlineCppFn_first(QList_i32 const &self);

int32_t const &inlineCppFn_last(QList_i32 const &self);

rust::Vec<int32_t> inlineCppFn_TraitImpl_From_ref_QList_i32_for_Vec_i32_from(QList_i32 const &src);

int32_t const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_i32_index(QList_i32 const &self,
                                                                             size_t index);

bool inlineCppFn_TraitImpl_PartialEq_array_of_i32_N_for_QList_i32_eq(
        QList_i32 const &self, rust::Slice<int32_t const> rhs);

} // namespace rust::bridge::qlist_i32

namespace rust {

template <>
struct IsRelocatable<::QList_i32> : ::std::true_type
{
};

} // namespace rust

#endif // _QLIST_I32_RUST_BRIDGE_H_
