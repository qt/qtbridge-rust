// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLIST_I16_RUST_BRIDGE_H_
#define _QLIST_I16_RUST_BRIDGE_H_

#include <QList>
#include <cstdint>
#include "rust/cxx.h"

using QList_i16 = ::QList<int16_t>;

namespace rust::bridge::qlist_i16 {

void QList_Drop(QList_i16 &v);
QList_i16 QList_Default();
QList_i16 QList_Clone(const QList_i16 &src);
bool QList_Eq(const QList_i16 &lhs, const QList_i16 &rhs);

void inlineCppFn_append(QList_i16 &self, int16_t value);

void inlineCppFn_clear(QList_i16 &self);

bool inlineCppFn_contains(QList_i16 const &self, int16_t const &value);

void inlineCppFn_push_back(QList_i16 &self, int16_t value);

void inlineCppFn_remove(QList_i16 &self, ptrdiff_t i, ptrdiff_t n);

void inlineCppFn_reserve(QList_i16 &self, size_t size);

ptrdiff_t inlineCppFn_size(QList_i16 const &self);

int16_t const &inlineCppFn_first(QList_i16 const &self);

int16_t const &inlineCppFn_last(QList_i16 const &self);

rust::Vec<int16_t> inlineCppFn_TraitImpl_From_ref_QList_i16_for_Vec_i16_from(QList_i16 const &src);

int16_t const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_i16_index(QList_i16 const &self,
                                                                             size_t index);

bool inlineCppFn_TraitImpl_PartialEq_array_of_i16_N_for_QList_i16_eq(
        QList_i16 const &self, rust::Slice<int16_t const> rhs);

} // namespace rust::bridge::qlist_i16

namespace rust {

template <>
struct IsRelocatable<::QList_i16> : ::std::true_type
{
};

} // namespace rust

#endif // _QLIST_I16_RUST_BRIDGE_H_
