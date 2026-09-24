// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use proc_macro2::{TokenStream};
use quote::{quote};
use syn::{parse::Parse, spanned::Spanned};

use crate::utils::parse_utils::parse_name_value;

pub struct QClassInfo{
    pub name: syn::LitStr,
    pub value: syn::LitStr,
}

impl QClassInfo {
    pub fn new(item: &syn::ImplItemMacro) -> syn::Result<Option<Self>> {
        if !item.mac.path.is_ident("qclass_info") {
            return Ok(None); // Not a 'qclass_info!' macro
        }

        if let Some(first_attr) = item.attrs.first() {
            return Err(syn::Error::new(first_attr.span(), "Attributes for qclass_info! macro are not supported"));
        }

        let class_info = item.mac.parse_body::<QClassInfo>()?;
        Ok(Some(class_info))
    }

    pub fn get_meta_registration_code(&self) -> syn::Result<TokenStream> {

        let QClassInfo {
            name,
            value,
            ..
        } = self;

        Ok(quote!{
                meta_obj.as_mut().add_class_info(#name, #value);
        })

    }

}

mod qclass_info_keywords {
    syn::custom_keyword!(Name);
    syn::custom_keyword!(Value);
}

impl syn::parse::Parse for QClassInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut value = None;

        while !input.is_empty() {
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
                if input.is_empty() { break; }
            }

            let token_begin = input.fork();
            if input.peek(qclass_info_keywords::Name) {
                read_attribute(input, &mut name, "Name")?;
            }
            else if input.peek(qclass_info_keywords::Value) {
                read_attribute(input, &mut value, "Value")?;
            }
            else {
                return Err(token_begin.error("Unsupported qclass_info attribute"));
            }
        }
        let name = name.ok_or_else(|| syn::Error::new(input.span(), "Missing Name attribute"))?;
        let value = value.ok_or_else(|| syn::Error::new(input.span(), "Missing Value attribute"))?;

        Ok(QClassInfo { name, value })
    }

}

fn read_attribute<T: Parse>(input: syn::parse::ParseStream, dst: &mut Option<T>, name: &'static str) -> syn::Result<()> {
    if dst.is_some() {
        return Err(syn::Error::new(input.span(), format!("'{name}' attribute is already defined for the property")));
    }

    *dst = Some(parse_name_value::<syn::Ident, T>(input)?.1);

    Ok(())
}
