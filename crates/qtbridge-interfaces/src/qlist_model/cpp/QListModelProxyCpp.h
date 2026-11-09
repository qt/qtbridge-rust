// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLISTMODELPROXYCPP_RUST_BRIDGE__H_
#define _QLISTMODELPROXYCPP_RUST_BRIDGE__H_
#include <QAbstractListModel>
#include <QByteArray>
#include <QHash>
#include <QMetaObject>
#include <QModelIndex>
#include <QVariant>
#include <QQmlListProperty>
#include <cstdint>
#include "qtbridge-interfaces/src/cpp/qbaseproxy.h"
#include "qtbridge-interfaces/src/qlist_model/proxy_rust_bridge.rs.h"

namespace rust::bridge {

class QListModelProxyCpp : public QAbstractListModel, public QBaseProxy<QListModelProxyCpp, QListModelProxyRust>
{
    using Base = QAbstractListModel;
public:
    QListModelProxyCpp(QListModelProxyRust* rustProxy);
    ~QListModelProxyCpp();

    // Virtual methods
    int32_t rowCount(const QModelIndex& parent) const override;
    QVariant data(const QModelIndex& index, int32_t role) const override;
    QHash<int32_t,QByteArray> roleNames() const override;
    bool setData(const QModelIndex& index, const QVariant& value, int32_t role) override;
    bool removeRows(int32_t first, int32_t count, const QModelIndex& parent) override;

    // Access to base implementation of non virtual functions
    // dataChanged needs a wrapper: the base signal has a defaulted roles argument.
    void dataChanged(const QModelIndex& topLeft, const QModelIndex& bottomRight);
    using Base::beginInsertRows;
    using Base::endInsertRows;
    using Base::beginMoveRows;
    using Base::endMoveRows;
    using Base::beginRemoveRows;
    using Base::endRemoveRows;
    using Base::beginResetModel;
    using Base::endResetModel;
};

} // namespace rust::bridge

#endif // _QLISTMODELPROXYCPP_RUST_BRIDGE__H_
