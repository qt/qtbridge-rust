// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qlist_ptr_mut_qobject.h"

namespace rust::bridge::qlist_ptr_mut_qobject {

void QList_Drop(QList_ptr_mut_QObject &v)
{
    v.~QList_ptr_mut_QObject();
}

QList_ptr_mut_QObject QList_Default()
{
    return QList_ptr_mut_QObject();
}

QList_ptr_mut_QObject QList_Clone(const QList_ptr_mut_QObject &src)
{
    return { src };
}

bool QList_Eq(const QList_ptr_mut_QObject &lhs, const QList_ptr_mut_QObject &rhs)
{
    return lhs == rhs;
}

void inlineCppFn_append(QList_ptr_mut_QObject &self, QObject *value)
{
    self.append(value);
}

void inlineCppFn_clear(QList_ptr_mut_QObject &self)
{
    self.clear();
}

bool inlineCppFn_contains(QList_ptr_mut_QObject const &self, QObject *const &value)
{
    return self.contains(value);
}

void inlineCppFn_push_back(QList_ptr_mut_QObject &self, QObject *value)
{
    self.push_back(value);
}

void inlineCppFn_remove(QList_ptr_mut_QObject &self, ptrdiff_t i, ptrdiff_t n)
{
    self.remove(i, n);
}

void inlineCppFn_reserve(QList_ptr_mut_QObject &self, size_t size)
{
    self.reserve(static_cast<qsizetype>(size));
}

ptrdiff_t inlineCppFn_size(QList_ptr_mut_QObject const &self)
{
    return self.size();
}

QObject *const &inlineCppFn_first(QList_ptr_mut_QObject const &self)
{
    return self.constFirst();
}

QObject *const &inlineCppFn_last(QList_ptr_mut_QObject const &self)
{
    return self.constLast();
}

QObject *const *inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_ptr_mut_QObject_index(
        QList_ptr_mut_QObject const &self, size_t index)
{
    return index < static_cast<size_t>(self.size()) ? &self[index] : nullptr;
}

bool inlineCppFn_TraitImpl_PartialEq_array_of_ptr_mut_QObject_N_for_QList_ptr_mut_QObject_eq(
        QList_ptr_mut_QObject const &self, rust::Slice<QObject *const> rhs)
{
    for (size_t i = 0; i < rhs.size(); ++i) {
        if (self[i] != rhs[i])
            return false;
    }
    return true;
}

} // namespace rust::bridge::qlist_ptr_mut_qobject
