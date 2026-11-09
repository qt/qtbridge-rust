// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QTABLEMODELPROXYCPP_RUST_BRIDGE__H_
#define _QTABLEMODELPROXYCPP_RUST_BRIDGE__H_
#include <QAbstractItemModel>
#include <QByteArray>
#include <QHash>
#include <QMetaObject>
#include <QModelIndex>
#include <QVariant>
#include <QQmlListProperty>
#include <cstdint>
#include "qtbridge-interfaces/src/cpp/qbaseproxy.h"
#include "qtbridge-interfaces/src/qtable_model/proxy_rust_bridge.rs.h"

namespace rust::bridge {

class QTableModelProxyCpp : public QAbstractItemModel, public QBaseProxy<QTableModelProxyCpp, QTableModelProxyRust>
{
    using Base = QAbstractItemModel;

public:
    QTableModelProxyCpp(QTableModelProxyRust* rustProxy);
    ~QTableModelProxyCpp();

    // Virtual methods
    QModelIndex index(int32_t row, int32_t column, const QModelIndex& parent) const override;
    QModelIndex parent(const QModelIndex& child) const override;
    int32_t rowCount(const QModelIndex& parent) const override;
    int32_t columnCount(const QModelIndex& parent) const override;
    QVariant data(const QModelIndex& index, int32_t role) const override;
    QHash<int32_t,QByteArray> roleNames() const override;
    bool setData(const QModelIndex& index, const QVariant& value, int32_t role) override;
    bool removeColumns(int32_t first, int32_t count, const QModelIndex& parent) override;
    bool removeRows(int32_t first, int32_t count, const QModelIndex& parent) override;

    // Access to base implementation of non virtual functions
    // dataChanged needs a wrapper: the base signal has a defaulted roles argument.
    void dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight);
    using Base::beginInsertColumns;
    using Base::endInsertColumns;
    using Base::beginInsertRows;
    using Base::endInsertRows;
    using Base::beginMoveColumns;
    using Base::endMoveColumns;
    using Base::beginMoveRows;
    using Base::endMoveRows;
    using Base::beginRemoveColumns;
    using Base::endRemoveColumns;
    using Base::beginRemoveRows;
    using Base::endRemoveRows;
    using Base::beginResetModel;
    using Base::endResetModel;
    // createIndex needs a wrapper: Qt's id type quintptr is not size_t on
    // every platform, which the bridge uses for usize.
    QModelIndex createIndex(int32_t row, int32_t column, size_t ptr) const;
};

} // namespace rust::bridge

#endif // _QTABLEMODELPROXYCPP_RUST_BRIDGE__H_
