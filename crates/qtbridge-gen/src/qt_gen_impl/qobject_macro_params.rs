// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use qtbridge_gen_common::parse_utils::parse_name_value;

#[derive(Default, Debug, Clone)]
pub struct QObjectMacroParams {
    pub base: Option<syn::Ident>,
    pub no_drop: bool,
    pub no_qml_element: bool,
    pub singleton: bool,
    pub link_me: bool,
    pub convert_to_camel_case: bool,
}

mod keywords {
    syn::custom_keyword!(Base);
    syn::custom_keyword!(NoDrop);
    syn::custom_keyword!(NoQmlElement);
    syn::custom_keyword!(Singleton);
    syn::custom_keyword!(LinkMe);
    syn::custom_keyword!(ConvertToCamelCase);
}

impl syn::parse::Parse for QObjectMacroParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut params = QObjectMacroParams::default();

        while !input.is_empty() {
            if input.peek(keywords::Base) {
                params.base = Some(parse_name_value::<keywords::Base, _>(input)?.1)
            } else if input.peek(keywords::NoDrop) {
                input.parse::<keywords::NoDrop>()?;
                params.no_drop = true;
            } else if input.peek(keywords::NoQmlElement) {
                if params.singleton {
                    return Err(input.error("NoQmlElement is not compatible with Singleton"))
                }
                input.parse::<keywords::NoQmlElement>()?;
                params.no_qml_element = true;
            } else if input.peek(keywords::Singleton) {
                if params.no_qml_element {
                    return Err(input.error("Singleton is not compatible with NoQmlElement"))
                }
                input.parse::<keywords::Singleton>()?;
                params.singleton = true;
            } else if input.peek(keywords::LinkMe) {
                input.parse::<keywords::LinkMe>()?;
                params.link_me = true;
            } else if input.peek(keywords::ConvertToCamelCase) {
                input.parse::<keywords::ConvertToCamelCase>()?;
                params.convert_to_camel_case = true;
            } else {
                return Err(input.error("Unsupported attribute of #[qobject] macro"))
            }
            let comma = input.parse::<Option<syn::Token![,]>>()?;
            if comma.is_none() && !input.is_empty() {
                return Err(input.error("Missing comma"))
            }
        }
        Ok(params)
    }
}

#[test]
fn singleton_conflict_error() {
    let result = syn::parse_str::<QObjectMacroParams>("NoQmlElement, Singleton");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not compatible"));
}

#[test]
fn parse_base() {
    let result = syn::parse_str::<QObjectMacroParams>("Base = QListModel");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().base.unwrap().to_string(), "QListModel");
}

#[test]
fn comma_separated_values() {
    let result = syn::parse_str::<QObjectMacroParams>("NoDrop Singleton");
    assert!(result.is_err());
}

#[test]
fn missing_comma_fails() {
    let params = syn::parse_str::<QObjectMacroParams>("NoDrop, Singleton").unwrap();
    assert_eq!(params.no_drop, true);
    assert_eq!(params.singleton, true);
}
