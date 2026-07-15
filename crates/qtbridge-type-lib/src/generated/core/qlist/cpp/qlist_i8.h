// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLIST_I8_RUST_BRIDGE_H_
#define _QLIST_I8_RUST_BRIDGE_H_

#include <QList>
#include <cstdint>
#include "rust/cxx.h"

using QList_i8 = ::QList<int8_t>;

namespace rust::bridge::qlist_i8 {

void QList_Drop(QList_i8 &v);
QList_i8 QList_Default();
QList_i8 QList_Clone(const QList_i8 &src);
bool QList_Eq(const QList_i8 &lhs, const QList_i8 &rhs);

void inlineCppFn_append(QList_i8 &self, int8_t value);

void inlineCppFn_clear(QList_i8 &self);

bool inlineCppFn_contains(QList_i8 const &self, int8_t const &value);

void inlineCppFn_push_back(QList_i8 &self, int8_t value);

void inlineCppFn_remove(QList_i8 &self, ptrdiff_t i, ptrdiff_t n);

void inlineCppFn_reserve(QList_i8 &self, size_t size);

ptrdiff_t inlineCppFn_size(QList_i8 const &self);

int8_t const &inlineCppFn_first(QList_i8 const &self);

int8_t const &inlineCppFn_last(QList_i8 const &self);

rust::Vec<int8_t> inlineCppFn_TraitImpl_From_ref_QList_i8_for_Vec_i8_from(QList_i8 const &src);

int8_t const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_i8_index(QList_i8 const &self,
                                                                           size_t index);

bool inlineCppFn_TraitImpl_PartialEq_array_of_i8_N_for_QList_i8_eq(QList_i8 const &self,
                                                                   rust::Slice<int8_t const> rhs);

} // namespace rust::bridge::qlist_i8

namespace rust {

template <>
struct IsRelocatable<::QList_i8> : ::std::true_type
{
};

} // namespace rust

#endif // _QLIST_I8_RUST_BRIDGE_H_
