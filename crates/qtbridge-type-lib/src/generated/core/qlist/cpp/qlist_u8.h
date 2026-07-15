// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLIST_U8_RUST_BRIDGE_H_
#define _QLIST_U8_RUST_BRIDGE_H_

#include <QList>
#include <cstdint>
#include "rust/cxx.h"

using QList_u8 = ::QList<uint8_t>;

namespace rust::bridge::qlist_u8 {

void QList_Drop(QList_u8 &v);
QList_u8 QList_Default();
QList_u8 QList_Clone(const QList_u8 &src);
bool QList_Eq(const QList_u8 &lhs, const QList_u8 &rhs);

void inlineCppFn_append(QList_u8 &self, uint8_t value);

void inlineCppFn_clear(QList_u8 &self);

bool inlineCppFn_contains(QList_u8 const &self, uint8_t const &value);

void inlineCppFn_push_back(QList_u8 &self, uint8_t value);

void inlineCppFn_remove(QList_u8 &self, ptrdiff_t i, ptrdiff_t n);

void inlineCppFn_reserve(QList_u8 &self, size_t size);

ptrdiff_t inlineCppFn_size(QList_u8 const &self);

uint8_t const &inlineCppFn_first(QList_u8 const &self);

uint8_t const &inlineCppFn_last(QList_u8 const &self);

rust::Vec<uint8_t> inlineCppFn_TraitImpl_From_ref_QList_u8_for_Vec_u8_from(QList_u8 const &src);

uint8_t const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_u8_index(QList_u8 const &self,
                                                                            size_t index);

bool inlineCppFn_TraitImpl_PartialEq_array_of_u8_N_for_QList_u8_eq(QList_u8 const &self,
                                                                   rust::Slice<uint8_t const> rhs);

} // namespace rust::bridge::qlist_u8

namespace rust {

template <>
struct IsRelocatable<::QList_u8> : ::std::true_type
{
};

} // namespace rust

#endif // _QLIST_U8_RUST_BRIDGE_H_
