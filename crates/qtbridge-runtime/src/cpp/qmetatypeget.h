// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#ifndef QMETA_TYPE_GET_H
#define QMETA_TYPE_GET_H

#include <QMetaType>

namespace rust::bridge::qmetatypeget {

template <typename T>
QMetaType QMetaTypeForType(const T *)
{
    return QMetaType::fromType<T>();
}

} // namespace rust::bridge::qmetatypeget

#endif // QMETA_TYPE_GET_H
