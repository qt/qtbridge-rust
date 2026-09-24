// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

pub fn snake_to_camel(input: &str) -> String {
    let mut result = String::new();
    for tok in input.split('_') {
        let mut chars = tok.chars();
        if let Some(first_char) = chars.next() {
            let first_char = if result.is_empty() {
                first_char.to_ascii_lowercase()
            } else {
                first_char.to_ascii_uppercase()
            };
            result.push(first_char);
            result.push_str(chars.as_str());
        }
    }

    result
}

pub fn camel_to_snake(input: &str) -> String {
    let mut result = String::new();
    let mut last_capital_pos: i32 = -1;
    let mut last_ch = '\0';

    for (pos, ch) in input.chars().enumerate() {
        let pos = pos as i32;
        let ch_lc = ch.to_ascii_lowercase();
        if ch_lc != ch {
            if pos - last_capital_pos > 1 && last_ch != '_' {
                result.push('_');
            }
            last_capital_pos = pos;
        }
        result.push(ch_lc);
        last_ch = ch_lc;
    }

    result
}

pub fn is_pascal_case(input: &str) -> bool {
    let Some(first_ch) = input.chars().next() else {
        return false
    };

    if first_ch.is_ascii_lowercase() {
        return false;
    }

    if input.chars()
        .any(|ch| ch == '_') {
        return false // TODO: return true in case like 'QAbstractItemModel_Proxy'
    }

    true
}
