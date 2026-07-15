// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLIST_F32_RUST_BRIDGE_H_
#define _QLIST_F32_RUST_BRIDGE_H_

#include <QList>
#include <cstdint>
#include "rust/cxx.h"

using QList_f32 = ::QList<float>;

namespace rust::bridge::qlist_f32 {

void QList_Drop(QList_f32 &v);
QList_f32 QList_Default();
QList_f32 QList_Clone(const QList_f32 &src);
bool QList_Eq(const QList_f32 &lhs, const QList_f32 &rhs);

void inlineCppFn_append(QList_f32 &self, float value);

void inlineCppFn_clear(QList_f32 &self);

bool inlineCppFn_contains(QList_f32 const &self, float const &value);

void inlineCppFn_push_back(QList_f32 &self, float value);

void inlineCppFn_remove(QList_f32 &self, ptrdiff_t i, ptrdiff_t n);

void inlineCppFn_reserve(QList_f32 &self, size_t size);

ptrdiff_t inlineCppFn_size(QList_f32 const &self);

float const &inlineCppFn_first(QList_f32 const &self);

float const &inlineCppFn_last(QList_f32 const &self);

rust::Vec<float> inlineCppFn_TraitImpl_From_ref_QList_f32_for_Vec_f32_from(QList_f32 const &src);

float const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_f32_index(QList_f32 const &self,
                                                                           size_t index);

bool inlineCppFn_TraitImpl_PartialEq_array_of_f32_N_for_QList_f32_eq(QList_f32 const &self,
                                                                     rust::Slice<float const> rhs);

} // namespace rust::bridge::qlist_f32

namespace rust {

template <>
struct IsRelocatable<::QList_f32> : ::std::true_type
{
};

} // namespace rust

#endif // _QLIST_F32_RUST_BRIDGE_H_
