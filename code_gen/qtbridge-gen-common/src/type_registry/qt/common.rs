// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use crate::naming;

pub fn get_include_path(path_in_gen: &str, submod_name: &str) -> Result<String, String> {
    let relative_path = path_in_gen
        .split("::")
        .collect::<Vec<_>>()
        .join("/");
    let filename = naming::cpp::filename::type_gen_header(submod_name);
    Ok(format!("qtbridge-type-lib/src/generated/{relative_path}/cpp/{filename}"))
}
