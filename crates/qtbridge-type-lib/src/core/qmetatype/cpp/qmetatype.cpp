// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qmetatype.h"

namespace rust::bridge::qmetatype {

QMetaType QMetaType_Default()
{
    return QMetaType();
}

bool QMetaType_Eq(const QMetaType &lhs, const QMetaType &rhs)
{
    return lhs == rhs;
}

QMetaType inlineCppFn_new(int32_t type_id)
{
    return QMetaType(type_id);
}

QMetaType inlineCppFn_new_with_interface(::QtPrivate::QMetaTypeInterface const *iface)
{
    return QMetaType(iface);
}

int32_t inlineCppFn_id(QMetaType const &self)
{
    return self.id();
}

bool inlineCppFn_is_valid(QMetaType const &self)
{
    return self.isValid();
}

rust::String inlineCppFn_name(QMetaType const &self)
{
    return CStrToRustString(self.name());
}

void inlineCppFn_register_type(QMetaType const &self)
{
    self.registerType();
}

} // namespace rust::bridge::qmetatype
