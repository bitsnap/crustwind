/*
 * Copyright (C) 2016-2025 Yuriy Yarosh
 * All rights reserved.
 *
 * SPDX-License-Identifier: MPL-2.0
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::css::units::Number;
use crate::css::units::color::Color;
use crate::css::units::color::ColorSpace;

/// Parse a hex color string into a Color
pub(crate) fn parse(hex: &str) -> Result<Color, anyhow::Error> {
    // Remove the leading #
    let hex = hex.trim_start_matches('#');

    // Parse based on the length of the hex string
    let (r, g, b, a) = match hex.len() {
        3 => {
            // #RGB format
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)? as i32;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)? as i32;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)? as i32;
            (r, g, b, 255)
        }
        4 => {
            // #RGBA format
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)? as i32;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)? as i32;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)? as i32;
            let a = u8::from_str_radix(&hex[3..4].repeat(2), 16)? as i32;
            (r, g, b, a)
        }
        6 => {
            // #RRGGBB format
            let r = u8::from_str_radix(&hex[0..2], 16)? as i32;
            let g = u8::from_str_radix(&hex[2..4], 16)? as i32;
            let b = u8::from_str_radix(&hex[4..6], 16)? as i32;
            (r, g, b, 255)
        }
        8 => {
            // #RRGGBBAA format
            let r = u8::from_str_radix(&hex[0..2], 16)? as i32;
            let g = u8::from_str_radix(&hex[2..4], 16)? as i32;
            let b = u8::from_str_radix(&hex[4..6], 16)? as i32;
            let a = u8::from_str_radix(&hex[6..8], 16)? as i32;
            (r, g, b, a)
        }
        _ => return Err(anyhow::anyhow!("Invalid hex color format")),
    };

    // Convert to Color struct with percentage values
    Ok(Color {
        space: ColorSpace::Srgb,
        c1: Number::from(r * 100 / 255),
        c2: Number::from(g * 100 / 255),
        c3: Number::from(b * 100 / 255),
        alpha: Number::from(a * 100 / 255),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_colors() {
        goldie::assert_json!(vec![
            parse("#F00").unwrap(),
            parse("#F00A").unwrap(),
            parse("#00FF00").unwrap(),
            parse("#0000FF80").unwrap(),
        ]);
    }

    #[test]
    fn test_parse_invalid_hex_colors() {
        goldie::assert_json!(
            vec![
                parse("#1").unwrap_err(),
                parse("#12345").unwrap_err(),
                parse("#GHIJKL").unwrap_err(),
            ]
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
        );
    }
}
