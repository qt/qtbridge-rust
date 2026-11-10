// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only
#ifndef QBASEPROXY_H
#define QBASEPROXY_H

#include <QQmlParserStatus>
#include <qqmlprivate.h>

#include "qtbridge-runtime/src/cpp/dispatchmetacallcpp.h"
#include "qtbridge-runtime/src/cpp/dynamicmetaobjectdata.h"
#include "qtbridge-runtime/src/cpp/rustobjectgetter.h"

template <typename Derived, typename RustProxy>
class QBaseProxy : public DispatchMetaCallCpp, public RustObjectGetter
{
protected:
    explicit QBaseProxy(RustProxy* rustProxy)
        : m_rustProxy(rustProxy)
    {}

public:
    ~QBaseProxy() override
    {
        RustProxy::dropSelf(m_rustProxy);
    }

    // DispatchMetaCallCpp implementation
    void invokeSlot(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const override
    {
        m_rustProxy->invokeSlot(slotId, inputs, outputs);
    }

    void invokeSlotMut(uint32_t slotId, rust::Slice<const uint8_t *const> inputs, rust::Slice<uint8_t* const> outputs) const override
    {
        m_rustProxy->invokeSlotMut(slotId, inputs, outputs);
    }

    QVariant readProperty(uint32_t propId) const override
    {
        return m_rustProxy->readProperty(propId);
    }

    void writeProperty(uint32_t propId, const QVariant& value) const override
    {
        m_rustProxy->writeProperty(propId, value);
    }

    void emitSignal(rust::Str signalName, rust::Slice<const uint8_t* const> argv) override
    {
        auto* self = static_cast<Derived*>(this);
        auto* meta = static_cast<DynamicMetaObjectData*>(QObjectPrivate::get(self)->metaObject);
        if (meta)
            meta->emitSignal(*self, signalName, argv);
        else
            qFatal() << "Error while emitting signal from Rust: The QObject does not contain a Rust dynamic meta object";
    }


    // RustObjectGetter implementation
    const void* getRustProxy() const override
    {
        return static_cast<const void*>(m_rustProxy);
    }

    // Static factory and query functions
    static Derived* create(RustProxy* rustProxy, const DynamicMetaObjectData* metaObject)
    {
        auto* proxy = new Derived(rustProxy);
        QObjectPrivate::get(proxy)->metaObject = const_cast<DynamicMetaObjectData*>(metaObject);
        return proxy;
    }

    static Derived* createAt(RustProxy* rustProxy, const DynamicMetaObjectData* metaObject, uint8_t* addr)
    {
        auto* proxy = new (addr) Derived(rustProxy);
        QObjectPrivate::get(proxy)->metaObject = const_cast<DynamicMetaObjectData*>(metaObject);
        return proxy;
    }

    static const QMetaObject& baseStaticMetaObject()
    {
        return Derived::staticMetaObject;
    }

    static size_t sizeOfProxy() { return sizeof(Derived); }
    static size_t alignOfProxy() { return alignof(Derived); }

    static int parserStatusCast()
    {
        return QQmlPrivate::StaticCastSelector<Derived, QQmlParserStatus>::cast();
    }

protected:
    RustProxy* m_rustProxy;
};

#endif // QBASEPROXY_H
