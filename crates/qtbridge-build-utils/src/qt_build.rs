// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::Command;
use qt_build_utils::{QtBuild, QtTool, QtToolQtPaths};

#[doc(hidden)]
pub struct QtInstallation {
    pub qt_paths: QtToolQtPaths,
    qt_build: QtBuild,
}

#[doc(hidden)]
impl Default for QtInstallation {
    fn default() -> Self {
        let qt = QtBuild::new(vec!["Core".to_owned()])
            .expect("Could not find Qt installation");
        let qt_paths = qt.qtpaths();
        Self { qt_paths, qt_build: qt }
    }
}

#[doc(hidden)]
impl QtInstallation {
    pub fn include_dirs(&self, modules: impl IntoIterator<Item: Display>, add_private: bool) -> Vec<String> {
        let qtbase_include_dir = PathBuf::from(self.qt_paths.query("QT_INSTALL_HEADERS").expect("qtpaths failed to query QT_INSTALL_HEADERS"));
        let qt_version = self.qt_paths.query("QT_VERSION").expect("qtpaths failed to query QT_VERSION");

        // With macOS framework installations/builds the header layout in each framework (eg. QtCore):
        //   Headers/                      public headers
        //   Headers/6.11/QtCore/private/  private headers
        //
        // Public headers are found by the compiler through -F framework search path, and makes
        // qualified includes work (<QtCore/qobject.h>). In addition we need explicit -I paths:
        // - private headers (<QtCore/private/qobject_p.h>)
        // - private headers may use relative includes (e.g. "qglobal.h") that require the
        //   unversioned Headers/ dir
        // - We still need to add QT_INSTALL_HEADERS in addition to framework header directories
        //   because some modules (e.g. QtQmlIntegration) ship only as plain include directories
        if self.is_qt_framework_installation() {
            let qt_libs_dir = self.qt_paths.query("QT_INSTALL_LIBS").expect("qtpaths failed to query QT_INSTALL_LIBS");
            let frameworks_path = PathBuf::from(qt_libs_dir);

            let mut dirs: Vec<String> = modules.into_iter()
                .flat_map(|module| {
                    let module_name = format!("Qt{module}");
                    let headers = frameworks_path
                        .join(format!("{module_name}.framework"))
                        .join("Headers");
                    if add_private {
                        let version_path = headers.join(&qt_version);
                        vec![
                            // Headers/6.11/
                            version_path.to_string_lossy().to_string(),
                            // Headers/6.11/QtCore/
                            version_path.join(&module_name).to_string_lossy().to_string(),
                            // Headers/
                            headers.to_string_lossy().to_string(),
                        ]
                    } else {
                        // Headers/
                        vec![headers.to_string_lossy().to_string()]
                    }
                })
                .collect();
            // include/
            dirs.push(qtbase_include_dir.to_string_lossy().to_string());
            return dirs;
        }

        modules.into_iter()
        .flat_map(|module| {
            let module_name = format!("Qt{module}");
            let module_path = qtbase_include_dir.join(&module_name);
            if add_private {
                let module_version_path = module_path.join(&qt_version);
                let private_module_path = module_version_path.join(&module_name);

                vec![module_version_path, private_module_path, module_path]
            } else {
                vec![module_path]
            }
        })
        .chain(std::iter::once(qtbase_include_dir.clone()))
        .map(|path| path.to_string_lossy().to_string())
        .collect()
    }


    pub fn configure_builder(&self, builder: &mut cc::Build) {
        self.set_platform_specific_flags(builder);
    }

    fn set_platform_specific_flags(&self, builder: &mut cc::Build) {
        if self.is_qt_framework_installation() {
            let qt_libs_dir = self.qt_paths.query("QT_INSTALL_LIBS").expect("qtpaths failed to query QT_INSTALL_LIBS");
            builder.flag(format!("-F{}", qt_libs_dir));
        }
    }

    fn is_qt_framework_installation(&self) -> bool {
        if !cfg!(target_os = "macos") {
            return false;
        }
        let qt_libs_dir = self.qt_paths.query("QT_INSTALL_LIBS").expect("qtpaths failed to query QT_INSTALL_LIBS");
        PathBuf::from(qt_libs_dir).join("QtCore.framework").exists()
    }

    pub fn link_modules(&self, modules: impl IntoIterator<Item: Display>) {
        let qt_libs_dir = self.qt_paths.query("QT_INSTALL_LIBS").expect("qtpaths failed to query QT_INSTALL_LIBS");

        if self.is_qt_framework_installation() {
            // On macOS Qt is typically installed as frameworks, not as plain libraries
            println!("cargo::rustc-link-search=framework={qt_libs_dir}");
            modules.into_iter()
                .for_each(|module| println!("cargo::rustc-link-lib=framework=Qt{module}"));
            return;
        }

        println!("cargo::rustc-link-search={qt_libs_dir}");

        modules.into_iter()
            .for_each(|module| {
                println!("cargo::rustc-link-lib=Qt6{module}")
        });
    }

    pub fn run_moc(&self, input: &Path, output: &Path) {
        let moc_path = self.qt_build.installation()
            .try_find_tool(QtTool::Moc)
            .expect("Could not find moc executable");

        println!("cargo::rerun-if-changed={}", input.display());

        let result = Command::new(&moc_path)
            .arg(input)
            .arg("-o")
            .arg(output)
            .output()
            .unwrap_or_else(|_| panic!("Failed to run moc for {}", input.display()));

        if !result.status.success() {
            panic!(
                "moc failed for {}:\n{}",
                input.display(),
                String::from_utf8_lossy(&result.stderr)
            );
        }
    }
}

pub fn get_cxx_qt_lib_manifest_dir() -> Result<PathBuf, String> {
    const MANIFEST_VAR_NAME: &str = "DEP_CXX_QT_LIB_CXX_QT_MANIFEST_PATH";
    let manifest_path_var = std::env::var(MANIFEST_VAR_NAME)
        .map_err(|err| format!("Failed to get environment variable '{MANIFEST_VAR_NAME}'. Error: '{err}'"))?;
    let path = PathBuf::from(&manifest_path_var)
        .parent()
        .ok_or_else(|| format!("Failed to get parent of manifest path '{manifest_path_var}'"))?
        .to_owned();
    Ok(path)
}

pub fn get_cxx_qt_lib_include_path() -> Result<PathBuf, String> {
    // Looks like there is no a metadata entry emitted by `cxx-qt-lib` that specifies headers location.
    // Therefore, we assume the headers are located in the 'include' subdirectory next to
    // the directory containing `manifest.json`.
    let path = get_cxx_qt_lib_manifest_dir()?
        .join("include");
    Ok(path)
}
