// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "qml_method_invoker.h"

#include "rustconv.h"

#include <QMetaMethod>

void connect_destroyed_callback(QObject *obj, std::uintptr_t flag_ptr)
{
    QObject::connect(obj, &QObject::destroyed, [flag_ptr](QObject *) {
        on_qobject_destroyed(flag_ptr);
    });
}

bool invoke_method(QObject *obj, rust::Str name, const QList<QVariant> &args)
{
    const QByteArray nameBa = RustStrToQByteArray(name);
    const QMetaObject *mo = obj->metaObject();
    const int argCount = static_cast<int>(args.size());
    if (argCount > 10)
        return false;

    // Traverse from the most-derived meta-object down to the base. Method
    // indices are laid out base-first, so scanning high -> low visits
    // QML-added members first, then Rust, then the C++ base. A candidate
    // matches when its name and parameter count match and every argument
    // actually converts to the corresponding parameter type; converting is the
    // selection test, so a candidate whose arguments don't convert is skipped
    // in favour of a less-derived one.
    for (int i = mo->methodCount() - 1; i >= 0; --i) {
        const QMetaMethod method = mo->method(i);
        if (method.name() != nameBa || method.parameterCount() != argCount)
            continue;

        QVariant converted[10];
        bool matches = true;
        for (int k = 0; k < argCount; ++k) {
            converted[k] = args.at(k);
            // canConvert() can be misleading: it reports whether a conversion
            // between the two types exists at all, not whether this particular
            // value converts. For example a QString may be convertible to int in
            // principle, but only convert() tells us whether the actual value
            // converts.
            if (!converted[k].convert(method.parameterMetaType(k))) {
                matches = false;
                break;
            }
        }
        if (!matches)
            continue;

        QGenericArgument gargs[10];
        for (int k = 0; k < argCount; ++k)
            gargs[k] = QGenericArgument(method.parameterMetaType(k).name(), converted[k].data());

        // Invoke by index so a same-named member cannot be re-resolved on the call.
        return method.invoke(obj, Qt::QueuedConnection, gargs[0], gargs[1], gargs[2],
                             gargs[3], gargs[4], gargs[5], gargs[6], gargs[7], gargs[8],
                             gargs[9]);
    }
    return false;
}
