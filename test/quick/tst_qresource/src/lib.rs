// Copyright (C) 2026 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge::{include_bytes_qml};

pub fn test_qresource() {
    use std::env;
    use std::path::PathBuf;

    include_bytes_qml!("example.txt");
    include_bytes_qml!("text/example.txt");
    include_bytes_qml!("example.txt", "samefolder");
    include_bytes_qml!("text/example.txt", "subfolder");
    include_bytes_qml!("example.txt", "samefolder/addedfolder");
    include_bytes_qml!("text/example.txt", "subfolder/addedfolder");

    include_bytes_qml!("dir");
    include_bytes_qml!("dir", "dirprefix");

    let mut input_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    input_path.push("qml");
    let input_folder = input_path.to_string_lossy().to_string();

    println!("Running quick test with qml files in \"{}\"", &input_folder);

    let args: Vec<String> = vec![
        file!().to_string(),
        "-input".into(),
        input_folder,
    ];

    let result = quicktest::quick_test_main(&args, &"test_qresource".into());
    assert_eq!(result, 0, "quick_test failed with code {}", result);
}
