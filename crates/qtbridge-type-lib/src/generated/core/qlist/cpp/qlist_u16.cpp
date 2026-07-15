// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qlist_u16.h"

namespace rust::bridge::qlist_u16 {

void QList_Drop(QList_u16 &v)
{
    v.~QList_u16();
}

QList_u16 QList_Default()
{
    return QList_u16();
}

QList_u16 QList_Clone(const QList_u16 &src)
{
    return { src };
}

bool QList_Eq(const QList_u16 &lhs, const QList_u16 &rhs)
{
    return lhs == rhs;
}

void inlineCppFn_append(QList_u16 &self, uint16_t value)
{
    self.append(value);
}

void inlineCppFn_clear(QList_u16 &self)
{
    self.clear();
}

bool inlineCppFn_contains(QList_u16 const &self, uint16_t const &value)
{
    return self.contains(value);
}

void inlineCppFn_push_back(QList_u16 &self, uint16_t value)
{
    self.push_back(value);
}

void inlineCppFn_remove(QList_u16 &self, ptrdiff_t i, ptrdiff_t n)
{
    self.remove(i, n);
}

void inlineCppFn_reserve(QList_u16 &self, size_t size)
{
    self.reserve(static_cast<qsizetype>(size));
}

ptrdiff_t inlineCppFn_size(QList_u16 const &self)
{
    return self.size();
}

uint16_t const &inlineCppFn_first(QList_u16 const &self)
{
    return self.constFirst();
}

uint16_t const &inlineCppFn_last(QList_u16 const &self)
{
    return self.constLast();
}

rust::Vec<uint16_t> inlineCppFn_TraitImpl_From_ref_QList_u16_for_Vec_u16_from(QList_u16 const &src)
{
    rust::Vec<uint16_t> result;
    result.reserve(static_cast<size_t>(src.size()));
    for (uint16_t item : src)
        result.push_back(item);
    return result;
}

uint16_t const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_u16_index(QList_u16 const &self,
                                                                              size_t index)
{
    return index < static_cast<size_t>(self.size()) ? &self[index] : nullptr;
}

bool inlineCppFn_TraitImpl_PartialEq_array_of_u16_N_for_QList_u16_eq(
        QList_u16 const &self, rust::Slice<uint16_t const> rhs)
{
    for (size_t i = 0; i < rhs.size(); ++i) {
        if (self[i] != rhs[i])
            return false;
    }
    return true;
}

} // namespace rust::bridge::qlist_u16
