// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::sync::LazyLock;
use qtbridge_build_utils::file_system_utils::{find_file_upwards, get_exe_dir, get_manifest_dir};
use crate::parse_utils::is_not_doc_attribute;
use proc_macro2::TokenStream;
use quote::ToTokens;

use syn::{visit_mut::VisitMut, Item, ImplItem};
struct StripDocs;

impl VisitMut for StripDocs {
    fn visit_item_mut(&mut self, item: &mut Item) {
        match item {
            Item::Fn(item_fn) => item_fn.attrs.retain(is_not_doc_attribute),
            Item::Struct(item_struct) => item_struct.attrs.retain(is_not_doc_attribute),
            Item::Enum(item_enum) => item_enum.attrs.retain(is_not_doc_attribute),
            Item::Impl(item_impl) => item_impl.attrs.retain(is_not_doc_attribute),
            Item::Mod(item_mod) => item_mod.attrs.retain(is_not_doc_attribute),
            Item::Const(item_const) => item_const.attrs.retain(is_not_doc_attribute),
            Item::Type(item_type) => item_type.attrs.retain(is_not_doc_attribute),
            Item::Macro(item_macro) => item_macro.attrs.retain(is_not_doc_attribute),
            _ => {}
        }

        syn::visit_mut::visit_item_mut(self, item);
    }

    fn visit_impl_item_mut(&mut self, item: &mut ImplItem) {
        if let ImplItem::Fn(item_fn) = item {
            item_fn.attrs.retain(is_not_doc_attribute)
        }
        syn::visit_mut::visit_impl_item_mut(self, item);
    }
}

/// Removes the documentation from the code. Useful for baseline tests and
/// compare code.
/// Note that this function does not strip all types of tokens but is
/// limited to the specific case of testing qobject.
/// TODO: Strip documentation of all types of tokens.
pub fn strip_docs(ts: TokenStream) -> TokenStream {
    let mut file: syn::File = syn::parse2(ts).unwrap();
    StripDocs.visit_file_mut(&mut file);
    file.to_token_stream()
}

pub fn format_rust_code(tokens: &TokenStream) -> Result<String, String> {

    // TODO: make code formatting optional (disable it if dedicated env variable is set) to save build time?

    let code = tokens.to_token_stream().to_string();
    // Run nightly rustfmt because it's needed for option 'normalize_doc_attributes' which is unstable so far.
    let output = run_cmd("rustfmt", &["+nightly", "--unstable-features", "--emit", "stdout"], &code)
        .map_err(|err| format!("Error running rustfmt:\n{err}"))?;
    Ok(output)
}

pub fn try_format_cpp_code(code: &str) -> Result<String, String> {
    static STYLE_FILE: LazyLock<Option<String>> = LazyLock::new(|| {
        find_clang_format_style_file()
    });

    let Some(style_file) = &*STYLE_FILE else {
        return Ok(code.into())
    };

    let result = run_cmd("clang-format", &[&format!("--style=file:{style_file}")], code);
    match result {
        Ok(output) => Ok(output),
        Err(RunCmdError::Io(_)) =>
            // Likely the reason of error is clang-format not in the PATH.
            // Ignore error since clang-format is currently optional.
            // Return code unformatted.
            Ok(code.to_owned()),
        Err(err) => Err(err.to_string()),
    }
}

/// Convert TokenStream to string with code
/// trying to remove unneeded spaces around '.' '::' ';' '<' '>'.
/// Those space are added in 'impl Display for TokenStream'.
/// Need to do this because not all of those unneeded spaces are eliminated by clang-fmt called later.
/// TODO: leave string literals unchanged (if we are within quotes).
pub fn token_stream_to_code(src: &TokenStream) -> String {

    let src = src.to_string();

    let mut result = String::with_capacity(src.len());
    let mut prev_ch = '\0';
    for ch in src.chars() {
        if ch == ' ' && prev_ch.is_ascii_punctuation() {
            continue; // remove space after punctuation
        }

        if ch.is_ascii_punctuation() && prev_ch == ' ' {
            result.pop(); // remove space before punctuation
        }

        result.push(ch);
        prev_ch = ch;
    }

    result
}


// Search for '.clang-format' file starting from the local folder of the package.
// If not found - check again a few levels up.
fn find_clang_format_style_file() -> Option<String> {

    // Start from the package root
    let manifest_dir = get_manifest_dir()
        .or_else(|_| get_exe_dir())
        .ok()?; // If get_manifest_dir() fails then probably called not from the build script

    find_file_upwards(&manifest_dir, ".clang-format", 5)
        .map(|path| path.to_string_lossy().to_string())
}

fn run_cmd(cmd_name: &str, args: &[&str], input: &str) -> Result<String, RunCmdError> {
    let mut cmd = Command::new(cmd_name)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let mut stdin = cmd.stdin.take()
            .ok_or_else(|| "Failed to open stdin".to_owned())?;
        stdin.write_all(input.as_bytes())?;
    }

    let output = cmd.wait_with_output()?;
    let output_str = String::from_utf8(output.stdout)
        .map_err(|err| format!("Failed to convert '{cmd_name}' command output to string:\n{err}"))?;
    Ok(output_str)
}

enum RunCmdError {
    Io(io::Error),
    Other(String),
}
impl From<io::Error> for RunCmdError {
    fn from(value: io::Error) -> Self {
        RunCmdError::Io(value)
    }
}
impl From<String> for RunCmdError {
    fn from(value: String) -> Self {
        RunCmdError::Other(value)
    }
}
impl std::fmt::Display for RunCmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunCmdError::Io(error) => write!(f, "{error}"),
            RunCmdError::Other(st) => write!(f, "{st}"),
        }
    }
}
