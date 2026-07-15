// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qlist_bool.h"

namespace rust::bridge::qlist_bool {

void QList_Drop(QList_bool &v)
{
    v.~QList_bool();
}

QList_bool QList_Default()
{
    return QList_bool();
}

QList_bool QList_Clone(const QList_bool &src)
{
    return { src };
}

bool QList_Eq(const QList_bool &lhs, const QList_bool &rhs)
{
    return lhs == rhs;
}

void inlineCppFn_append(QList_bool &self, bool value)
{
    self.append(value);
}

void inlineCppFn_clear(QList_bool &self)
{
    self.clear();
}

bool inlineCppFn_contains(QList_bool const &self, bool const &value)
{
    return self.contains(value);
}

void inlineCppFn_push_back(QList_bool &self, bool value)
{
    self.push_back(value);
}

void inlineCppFn_remove(QList_bool &self, ptrdiff_t i, ptrdiff_t n)
{
    self.remove(i, n);
}

void inlineCppFn_reserve(QList_bool &self, size_t size)
{
    self.reserve(static_cast<qsizetype>(size));
}

ptrdiff_t inlineCppFn_size(QList_bool const &self)
{
    return self.size();
}

bool const &inlineCppFn_first(QList_bool const &self)
{
    return self.constFirst();
}

bool const &inlineCppFn_last(QList_bool const &self)
{
    return self.constLast();
}

rust::Vec<bool> inlineCppFn_TraitImpl_From_ref_QList_bool_for_Vec_bool_from(QList_bool const &src)
{
    rust::Vec<bool> result;
    result.reserve(static_cast<size_t>(src.size()));
    for (bool item : src)
        result.push_back(item);
    return result;
}

bool const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_bool_index(QList_bool const &self,
                                                                           size_t index)
{
    return index < static_cast<size_t>(self.size()) ? &self[index] : nullptr;
}

bool inlineCppFn_TraitImpl_PartialEq_array_of_bool_N_for_QList_bool_eq(QList_bool const &self,
                                                                       rust::Slice<bool const> rhs)
{
    for (size_t i = 0; i < rhs.size(); ++i) {
        if (self[i] != rhs[i])
            return false;
    }
    return true;
}

} // namespace rust::bridge::qlist_bool
