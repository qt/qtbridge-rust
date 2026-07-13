// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::QMetaType;

#[qt_gen::bridge]
mod qmetaobject {
    include_in_cpp!(<QMetaObject>);
    include_in_cpp!(<QMetaType>);

    /// The QMetaObject struct contains meta-information about Qt objects.
    ///
    /// See also: [QMetaObject documentation](https://doc.qt.io/qt-6/qmetaobject.html).
    struct QMetaObject;

    /// Returns the metatype corresponding to this metaobject.
    pub fn meta_type(&self) -> QMetaType {
        let cpp = cpp_fn!(|&self| -> QMetaType {
            return self.metaType();
        });
        cpp(self)
    }

    /// Returns `true` if the class described by this QMetaObject
    /// inherits the type described by `base`.
    pub fn inherits(&self, base: &QMetaObject) -> bool {
        let cpp = cpp_fn!(|&self, base: &QMetaObject| -> bool {
            return self.inherits(&base);
        });
        cpp(self, base)
    }

    pub fn invoke_method(obj: *mut QObject, name: &str) -> bool {
        let cpp = cpp_fn!(|obj: *mut QObject, name: &str| -> bool {
            QByteArray nameBa = RustStrToQByteArray(name);
            return QMetaObject::invokeMethod(obj, nameBa.constData(), Qt::QueuedConnection);
        });
        unsafe { cpp(obj, name) }
    }

    pub fn invoke_method_with_args(obj: *mut QObject, name: &str, args: &QVariantList) -> bool {
        let cpp = cpp_fn!(|obj: *mut QObject, name: &str, args: &QVariantList| -> bool {
            const QByteArray nameBa = RustStrToQByteArray(name);
            const QMetaObject *metaObj = obj->metaObject();

            // check that parameter count matches and doesn't exceed 10
            int methodIndex = -1;
            for (int i = 0; i < metaObj->methodCount(); ++i) {
                if (metaObj->method(i).name() == nameBa) { methodIndex = i; break; }
            }
            if (methodIndex < 0) return false;
            const QMetaMethod method = metaObj->method(methodIndex);
            const int paramCount = method.parameterCount();
            if (args.size() < paramCount || paramCount > 10) return false;

            // check argument types and create QGenericArguments
            QGenericArgument gargs[10];
            QVariant arg[10];
            for (int i = 0; i < paramCount; ++i) {
                const QMetaType targetType = method.parameterMetaType(i);
                arg[i] = args.at(i);
                if (!arg[i].convert(targetType)) return false;
                gargs[i] = QGenericArgument(targetType.name(), arg[i].data());
            }

            return QMetaObject::invokeMethod(obj, nameBa.constData(), Qt::QueuedConnection,
                gargs[0], gargs[1], gargs[2], gargs[3], gargs[4],
                gargs[5], gargs[6], gargs[7], gargs[8], gargs[9]);
        });
        unsafe { cpp(obj, name, args) }
    }
}
