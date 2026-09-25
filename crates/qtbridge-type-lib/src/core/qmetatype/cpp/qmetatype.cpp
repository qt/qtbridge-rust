// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qmetatype.h"
#include "rustconv.h"

namespace rust::bridge::qmetatype {

QMetaType defaultQMetaType()
{
    return QMetaType();
}

QMetaType newWithInterface(::QtPrivate::QMetaTypeInterface const &iface)
{
    return QMetaType(&iface);
}

bool eq(QMetaType const &lhs, QMetaType const &rhs)
{
    return lhs == rhs;
}

int32_t id(QMetaType const &obj)
{
    return obj.id();
}

rust::String name(QMetaType const &obj)
{
    return CStrToRustString(obj.name());
}

void registerType(QMetaType const &obj)
{
    obj.registerType();
}

} // namespace rust::bridge::qmetatype
