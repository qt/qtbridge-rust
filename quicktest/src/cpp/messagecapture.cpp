// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "messagecapture.h"

#include "rustconv.h"

#include <QtGlobal>
#include <QMutex>
#include <QString>
#include <QStringList>

#include <mutex>

namespace {

QMutex capture_mutex;
QStringList messages;
QtMessageHandler previousHandler = nullptr;

void captureHandler(QtMsgType type, const QMessageLogContext &context, const QString &msg)
{
    {
        const QMutexLocker lock(&capture_mutex);
        messages.append(msg);
    }
    // Forward to the previous handler (usually the default one) so messages
    // still reach the debug output, log files, etc.
    if (previousHandler)
        previousHandler(type, context, msg);
}

} // namespace

namespace rust::bridge
{
    void installMessageCapture()
    {
        const QtMessageHandler previous = qInstallMessageHandler(captureHandler);
        // Guard against installing twice: chaining to ourselves would recurse.
        if (previous != captureHandler)
            previousHandler = previous;
    }

    rust::Vec<rust::String> takeCapturedMessages()
    {
        QMutexLocker lock(&capture_mutex);
        QStringList captured = messages;
        messages.clear();
        lock.unlock();
        return QStringListToRustStringList(captured);
    }

} // namespace rust::bridge
