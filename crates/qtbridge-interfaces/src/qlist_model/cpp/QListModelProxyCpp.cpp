// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "QListModelProxyCpp.h"

namespace rust::bridge {

QListModelProxyCpp::QListModelProxyCpp(QListModelProxyRust* rustProxy)
    : QBaseProxy(rustProxy)
{}
QListModelProxyCpp::~QListModelProxyCpp() = default;

// Virtual methods
int32_t QListModelProxyCpp::rowCount(const QModelIndex& parent) const
{
    return m_rustProxy->rowCount(parent);
}
QVariant QListModelProxyCpp::data(const QModelIndex& index, int32_t role) const
{
    return m_rustProxy->data(index, role);
}
QHash<int32_t,QByteArray> QListModelProxyCpp::roleNames() const
{
    return m_rustProxy->roleNames();
}
bool QListModelProxyCpp::setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return m_rustProxy->setData(index, value, role);
}
bool QListModelProxyCpp::removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return m_rustProxy->removeRows(first, count, parent);
}

// Access to base implementation of non virtual functions
void QListModelProxyCpp::dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight)
{
    Base::dataChanged(topLeft, bottomRight);
}

} // namespace rust::bridge
