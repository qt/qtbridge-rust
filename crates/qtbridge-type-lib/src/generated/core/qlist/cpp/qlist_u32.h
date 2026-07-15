// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLIST_U32_RUST_BRIDGE_H_
#define _QLIST_U32_RUST_BRIDGE_H_

#include <QList>
#include <cstdint>
#include "rust/cxx.h"

using QList_u32 = ::QList<uint32_t>;

namespace rust::bridge::qlist_u32 {

void QList_Drop(QList_u32 &v);
QList_u32 QList_Default();
QList_u32 QList_Clone(const QList_u32 &src);
bool QList_Eq(const QList_u32 &lhs, const QList_u32 &rhs);

void inlineCppFn_append(QList_u32 &self, uint32_t value);

void inlineCppFn_clear(QList_u32 &self);

bool inlineCppFn_contains(QList_u32 const &self, uint32_t const &value);

void inlineCppFn_push_back(QList_u32 &self, uint32_t value);

void inlineCppFn_remove(QList_u32 &self, ptrdiff_t i, ptrdiff_t n);

void inlineCppFn_reserve(QList_u32 &self, size_t size);

ptrdiff_t inlineCppFn_size(QList_u32 const &self);

uint32_t const &inlineCppFn_first(QList_u32 const &self);

uint32_t const &inlineCppFn_last(QList_u32 const &self);

rust::Vec<uint32_t> inlineCppFn_TraitImpl_From_ref_QList_u32_for_Vec_u32_from(QList_u32 const &src);

uint32_t const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_u32_index(QList_u32 const &self,
                                                                              size_t index);

bool inlineCppFn_TraitImpl_PartialEq_array_of_u32_N_for_QList_u32_eq(
        QList_u32 const &self, rust::Slice<uint32_t const> rhs);

} // namespace rust::bridge::qlist_u32

namespace rust {

template <>
struct IsRelocatable<::QList_u32> : ::std::true_type
{
};

} // namespace rust

#endif // _QLIST_U32_RUST_BRIDGE_H_
