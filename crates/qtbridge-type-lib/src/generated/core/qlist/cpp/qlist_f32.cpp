// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qlist_f32.h"

namespace rust::bridge::qlist_f32 {

void QList_Drop(QList_f32 &v)
{
    v.~QList_f32();
}

QList_f32 QList_Default()
{
    return QList_f32();
}

QList_f32 QList_Clone(const QList_f32 &src)
{
    return { src };
}

bool QList_Eq(const QList_f32 &lhs, const QList_f32 &rhs)
{
    return lhs == rhs;
}

void inlineCppFn_append(QList_f32 &self, float value)
{
    self.append(value);
}

void inlineCppFn_clear(QList_f32 &self)
{
    self.clear();
}

bool inlineCppFn_contains(QList_f32 const &self, float const &value)
{
    return self.contains(value);
}

void inlineCppFn_push_back(QList_f32 &self, float value)
{
    self.push_back(value);
}

void inlineCppFn_remove(QList_f32 &self, ptrdiff_t i, ptrdiff_t n)
{
    self.remove(i, n);
}

void inlineCppFn_reserve(QList_f32 &self, size_t size)
{
    self.reserve(static_cast<qsizetype>(size));
}

ptrdiff_t inlineCppFn_size(QList_f32 const &self)
{
    return self.size();
}

float const &inlineCppFn_first(QList_f32 const &self)
{
    return self.constFirst();
}

float const &inlineCppFn_last(QList_f32 const &self)
{
    return self.constLast();
}

rust::Vec<float> inlineCppFn_TraitImpl_From_ref_QList_f32_for_Vec_f32_from(QList_f32 const &src)
{
    rust::Vec<float> result;
    result.reserve(static_cast<size_t>(src.size()));
    for (float item : src)
        result.push_back(item);
    return result;
}

float const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_f32_index(QList_f32 const &self,
                                                                           size_t index)
{
    return index < static_cast<size_t>(self.size()) ? &self[index] : nullptr;
}

bool inlineCppFn_TraitImpl_PartialEq_array_of_f32_N_for_QList_f32_eq(QList_f32 const &self,
                                                                     rust::Slice<float const> rhs)
{
    for (size_t i = 0; i < rhs.size(); ++i) {
        if (self[i] != rhs[i])
            return false;
    }
    return true;
}

} // namespace rust::bridge::qlist_f32
