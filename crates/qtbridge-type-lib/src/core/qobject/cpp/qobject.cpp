// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qobject.h"

#include <string>

namespace rust::bridge::qobject {

const QMetaObject *metaObject(const QObject &obj)
{
    return obj.metaObject();
}

void deleteQObject(QObject *obj)
{
    delete obj;
}

void destructQObject(QObject *obj)
{
    obj->~QObject();
}

QVariant property(const QObject &obj, rust::Str name)
{
    return obj.property(std::string(name).c_str());
}

bool setProperty(QObject *obj, rust::Str name, const QVariant &value)
{
    return obj->setProperty(std::string(name).c_str(), value);
}

} // namespace rust::bridge::qobject
