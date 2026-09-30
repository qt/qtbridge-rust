// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "quicktestmain.h"
//#include "rustconv.h"

#include <QtQuickTest>
#include <QQmlEngine>
#include <QQmlContext>

#include <vector>
#include <string>

#include "qtestsetup.h"

namespace rust::bridge
{
    int quickTestMainWithSetup(const rust::Vec<rust::String>& args, const rust::String &name, QObject *setup) {

        std::vector<std::string> storage = {args.begin(), args.end()};
        std::vector<char*> argv;
        for (auto& str : storage)
            argv.push_back(str.data());
        int argc = static_cast<int>(storage.size());

        return quick_test_main_with_setup(argc, argv.data(), std::string(name).data(), nullptr, setup);
    }

    int quickTestMain(const rust::Vec<rust::String>& args, const rust::String &name) {
        return quickTestMainWithSetup(args, name, nullptr);
    }

    int quickTestMainWithProperties(const rust::Vec<rust::String>& args, const rust::String &name, const QVariantMap &properties)
    {
        QtQuickTestSetup setup(properties);
        return quickTestMainWithSetup(args, name, &setup);
    }

} // namespace rust::bridge
