// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "QTableModelProxyCpp.h"

namespace rust::bridge {

QTableModelProxyCpp::QTableModelProxyCpp(QTableModelProxyRust* rustProxy)
    : QBaseProxy(rustProxy)
{}
QTableModelProxyCpp::~QTableModelProxyCpp() = default;

// Virtual methods
QModelIndex QTableModelProxyCpp::index(int32_t row, int32_t column, const QModelIndex&) const
{
    return Base::createIndex(row, column);
}
QModelIndex QTableModelProxyCpp::parent(const QModelIndex&) const
{
    return {};
}
int32_t QTableModelProxyCpp::rowCount(const QModelIndex& parent) const
{
    return m_rustProxy->rowCount(parent);
}
int32_t QTableModelProxyCpp::columnCount(const QModelIndex& parent) const
{
    return m_rustProxy->columnCount(parent);
}
QVariant QTableModelProxyCpp::data(const QModelIndex& index, int32_t role) const
{
    return m_rustProxy->data(index, role);
}
QHash<int32_t,QByteArray> QTableModelProxyCpp::roleNames() const
{
    return m_rustProxy->roleNames();
}
bool QTableModelProxyCpp::setData(const QModelIndex& index, const QVariant& value, int32_t role)
{
    return m_rustProxy->setData(index, value, role);
}
bool QTableModelProxyCpp::removeColumns(int32_t first, int32_t count, const QModelIndex& parent)
{
    return m_rustProxy->removeColumns(first, count, parent);
}
bool QTableModelProxyCpp::removeRows(int32_t first, int32_t count, const QModelIndex& parent)
{
    return m_rustProxy->removeRows(first, count, parent);
}

// Access to base implementation of non virtual functions
void QTableModelProxyCpp::dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight)
{
    Base::dataChanged(topLeft, bottomRight);
}
QModelIndex QTableModelProxyCpp::createIndex(int32_t row, int32_t column, size_t ptr) const
{
    return Base::createIndex(row, column, ptr);
}

} // namespace rust::bridge
