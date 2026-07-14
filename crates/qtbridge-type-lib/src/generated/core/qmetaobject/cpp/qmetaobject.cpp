// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qmetaobject.h"
#include "rustconv.h"

namespace rust::bridge::qmetaobject {

QMetaType inlineCppFn_meta_type(QMetaObject const &self)
{
    return self.metaType();
}

bool inlineCppFn_inherits(QMetaObject const &self, QMetaObject const &base)
{
    return self.inherits(&base);
}

} // namespace rust::bridge::qmetaobject
