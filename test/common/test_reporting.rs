// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

macro_rules! run_test {
    ($test:path) => {
        match std::panic::catch_unwind($test) {
            Ok(()) => eprintln!("test {} ... ok", stringify!($test)),
            Err(panic) => {
                eprintln!("test {} ... FAILED", stringify!($test));
                std::panic::resume_unwind(panic);
            }
        }
    };
}

pub(crate) use run_test;
