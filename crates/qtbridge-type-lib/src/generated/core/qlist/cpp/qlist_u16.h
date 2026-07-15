// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLIST_U16_RUST_BRIDGE_H_
#define _QLIST_U16_RUST_BRIDGE_H_

#include <QList>
#include <cstdint>
#include "rust/cxx.h"

using QList_u16 = ::QList<uint16_t>;

namespace rust::bridge::qlist_u16 {

void QList_Drop(QList_u16 &v);
QList_u16 QList_Default();
QList_u16 QList_Clone(const QList_u16 &src);
bool QList_Eq(const QList_u16 &lhs, const QList_u16 &rhs);

void inlineCppFn_append(QList_u16 &self, uint16_t value);

void inlineCppFn_clear(QList_u16 &self);

bool inlineCppFn_contains(QList_u16 const &self, uint16_t const &value);

void inlineCppFn_push_back(QList_u16 &self, uint16_t value);

void inlineCppFn_remove(QList_u16 &self, ptrdiff_t i, ptrdiff_t n);

void inlineCppFn_reserve(QList_u16 &self, size_t size);

ptrdiff_t inlineCppFn_size(QList_u16 const &self);

uint16_t const &inlineCppFn_first(QList_u16 const &self);

uint16_t const &inlineCppFn_last(QList_u16 const &self);

rust::Vec<uint16_t> inlineCppFn_TraitImpl_From_ref_QList_u16_for_Vec_u16_from(QList_u16 const &src);

uint16_t const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_u16_index(QList_u16 const &self,
                                                                              size_t index);

bool inlineCppFn_TraitImpl_PartialEq_array_of_u16_N_for_QList_u16_eq(
        QList_u16 const &self, rust::Slice<uint16_t const> rhs);

} // namespace rust::bridge::qlist_u16

namespace rust {

template <>
struct IsRelocatable<::QList_u16> : ::std::true_type
{
};

} // namespace rust

#endif // _QLIST_U16_RUST_BRIDGE_H_
