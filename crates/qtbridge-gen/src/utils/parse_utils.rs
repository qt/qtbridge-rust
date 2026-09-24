// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use syn::parse::{Parse, ParseBuffer};

struct NameEqValue<Name, Value> {
    pub name: Name,
    pub value: Value,
}

impl<Name: Parse, Value: Parse> syn::parse::Parse for NameEqValue<Name, Value> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _eq: syn::Token![=] = input.parse()?;
        let value = input.parse()?;
        Ok(NameEqValue { name, value })
    }
}

pub fn parse_name_value<Name: Parse, Value: Parse>(input: &ParseBuffer) -> syn::Result<(Name, Value)> {
    let begin = input.fork();
    let nv = match input.parse::<NameEqValue<Name, Value>>() {
        Ok(nv) => nv,
        Err(_) => return Err(begin.error("Failed to parse expression like name=value")),
    };

    Ok((nv.name, nv.value))
}

pub fn partition_attr_by(attrs: Vec<syn::Attribute>, pred: fn (&syn::Attribute)->bool) -> (Vec<syn::Attribute>, Option<syn::Attribute>) {

    if let Some(pos) = attrs.iter().position(pred) {
        let mut attrs = attrs;
        let found = attrs.remove(pos);
        return (attrs, Some(found));
    }

    (attrs, None)
}

#[cfg(test)]
pub fn is_doc_attribute(attr: &syn::Attribute) -> bool {
    if let syn::AttrStyle::Outer = attr.style &&
       let Some(ident) = attr.path().get_ident() &&
       ident == "doc"
    {
        return true
    }

    false
}

#[cfg(test)]
pub fn is_not_doc_attribute(attr: &syn::Attribute) -> bool {
    !is_doc_attribute(attr)
}
