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

#[cfg(test)]
mod tests {
    use super::{camel_to_snake, snake_to_camel};

    #[test]
    pub fn require_that_camel_to_snake_returns_output_that_agrees_with_reference() {
        let cases = [
            ("", ""),
            ("one", "one"),
            ("oneTwoThree", "one_two_three"),
            ("FirstCapital", "first_capital"),
            ("already_in_snake", "already_in_snake"),
            ("capitalAfter_Underscore", "capital_after_underscore"),
            ("two__underscores_in_snake", "two__underscores_in_snake"),
            ("RGB", "rgb"),
            ("getResultAsQVariant", "get_result_as_qvariant"),
        ];

        for (src, expected) in cases {
            let actual = camel_to_snake(src);
            assert_eq!(actual, expected)
        }
    }

    #[test]
    pub fn require_that_snake_to_camel_returns_output_that_agrees_with_reference() {
        let cases = [
            ("", ""),
            ("one", "one"),
            ("one_two_three", "oneTwoThree"),
            ("alreadyInCamel", "alreadyInCamel"),
        ];

        for (src, expected) in cases {
            let actual = snake_to_camel(src);
            assert_eq!(actual, expected)
        }
    }
}
