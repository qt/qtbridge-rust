// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "dynamicmetaobjectbuilder.h"
#include "dynamicmetaobjectdata.h"
#include "rustconv.h"
#include <QMetaType>
#include <QScopedPointer>
#include <QSpan>
#include <QtLogging>
#include <private/qmetaobjectbuilder_p.h>
#include <optional>

namespace
{
    QByteArray generateFuncSignature(const QByteArray& name, const QSpan<const QMetaType>& argMetaTypes)
    {
        QByteArray paramStr;
        for (const auto& type : argMetaTypes)
        {
            if (!type.isValid())
                qFatal() << "Unspecified argument type";

            if (!paramStr.isEmpty())
                paramStr.append(',');
            paramStr += type.name();
        }

        QByteArray sign = name + '(' + paramStr + ')';
        return QMetaObject::normalizedSignature(sign.constData());
    }

    // There are behavior changes in QMetaObjectBuilder between Qt versions 6.11 and 6.12.
    // After that change, we need to call the newly introduced QMetaObject::signal(),
    // otherwise, for earlier versions, call QMetaObject::method().
    // Meta methods in QMetaObject ordered such that signals go first,
    // so calling method(i) is equivalent to signal(i) when i is within signal count.
    // TODO: switch to `#if QT_VERSION >= QT_VERSION_CHECK(6, 12, 0)` later.
    template<typename, typename = void>
    struct has_signal_method : std::false_type {};

    template<typename T>
    struct has_signal_method<T, std::void_t<decltype(std::declval<T>().signal(0))>>
    : std::true_type {};

    template <typename T>
    QMetaMethodBuilder getSignal(const T& src, int index)
    {
        QMetaMethodBuilder result;
        if constexpr (has_signal_method<T>::value)
            result = src.signal(index);
        else
        {
            result = src.method(index);
            if (result.methodType() != QMetaMethod::Signal)
                qFatal() << "Logic error: " << result.signature() << " expected to be a signal";
        }

        return result;
    }
} // namespace anonymous


DynamicMetaObjectBuilder::DynamicMetaObjectBuilder(const QMetaObject* staticMetaObj, rust::Str className)
    : m_mob(std::make_unique<QMetaObjectBuilder>())
    , m_data(std::make_unique<DynamicMetaObjectData>())
{
    m_mob->setSuperClass(staticMetaObj); // TODO: check without this
    m_mob->setClassName(className.empty() ? QByteArray(staticMetaObj->className()) : RustStrToQByteArray(className));
}

DynamicMetaObjectBuilder::~DynamicMetaObjectBuilder()
{}

void DynamicMetaObjectBuilder::addClassInfo(rust::Str name, rust::Str value)
{
    m_mob->addClassInfo(RustStrToQByteArray(name), RustStrToQByteArray(value));
}

void DynamicMetaObjectBuilder::registerProperty(rust::Str name, uint32_t propId, const QMetaType& metaType, bool isConstant, rust::Str notifySignal)
{
    if (!m_mob)
        qFatal() << "Property registration must be done before endMetaRegistration() call";

    QByteArray nameBa = RustStrToQByteArray(name);

    if (!metaType.isValid())
        qFatal() << "Invalid type of property " << nameBa;

    std::optional<int> signalIndex;
    if (!notifySignal.empty())
    {
        QByteArray notifySignalBa = RustStrToQByteArray(notifySignal);
        signalIndex = m_data->getSignalIndex(notifySignalBa);
        if (!signalIndex)
            qFatal() << "Failed to find a signal by name: " << notifySignalBa;
    }

    const bool writable = !isConstant;
    metaType.registerType();

    QMetaPropertyBuilder builder = m_mob->addProperty(nameBa, metaType.name());
    builder.setReadable(true);
    builder.setWritable(writable);
    builder.setConstant(isConstant);

    if (signalIndex)
        builder.setNotifySignal(getSignal(*m_mob, *signalIndex));

    m_data->addProperty(propId, metaType);
}

void DynamicMetaObjectBuilder::registerSignal(rust::Str name, rust::Slice<const QMetaType> argMetaTypes)
{
    if (!m_mob)
        qFatal() << "Signal registration must be done before endMetaRegistration() call";

    for (const QMetaType& type: argMetaTypes)
        type.registerType();

    QByteArray nameBa = RustStrToQByteArray(name);
    QByteArray signature = generateFuncSignature(nameBa, argMetaTypes);
    m_mob->addSignal(signature);
    m_data->addSignal(nameBa);
}

void DynamicMetaObjectBuilder::registerSlot(rust::Str name, uint32_t slotId, rust::Slice<const QMetaType> argMetaTypes, const QMetaType& returnMetaType, Mutability mutability)
{
    QByteArray nameBa = RustStrToQByteArray(name);

    if (!m_mob)
        qFatal() << "Failed to register slot " << nameBa << ". Slot registration must be done before endMetaRegistration() call";

    for (const QMetaType& type: argMetaTypes)
        type.registerType();


    QByteArray signature = generateFuncSignature(nameBa, argMetaTypes);
    QMetaMethodBuilder builder = m_mob->addSlot(signature);
    if (returnMetaType.isValid()) {
        returnMetaType.registerType();
        builder.setReturnType(returnMetaType.name());
    }
    m_data->addSlot(slotId, mutability);
}

void DynamicMetaObjectBuilder::endMetaRegistration()
{
    if (m_mob)
    {
        std::unique_ptr<QMetaObject, QScopedPointerPodDeleter> metaObject(m_mob->toMetaObject());
        m_data->setMetaObject(std::move(metaObject));
        m_mob.reset();
    }
    else
    {
        qFatal() << __func__ << "() is called more than once";
    }
}

const DynamicMetaObjectData* DynamicMetaObjectBuilder::takeDynamicMetaObjectData()
{
    return m_data.release();
}

std::unique_ptr<DynamicMetaObjectBuilder> createDynamicMetaObjectBuilder(rust::Str rustStructName, const QMetaObject& staticMeta)
{
    return std::make_unique<DynamicMetaObjectBuilder>(&staticMeta, rustStructName);
}
