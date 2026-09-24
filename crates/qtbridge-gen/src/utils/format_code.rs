// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use std::io::{self, Write};
use std::process::{Command, Stdio};
use crate::utils::parse_utils::is_not_doc_attribute;
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
