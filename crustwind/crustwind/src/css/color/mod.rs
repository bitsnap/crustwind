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
use derive_more::*;
use std::str::FromStr;

use serde::{Serialize, Deserialize};

use crate::css::units::Number;

#[derive(Clone, Copy, Debug, PartialEq, Display, From, FromStr, Serialize, Deserialize)]
pub enum ColorSpace {
    #[display("srgb")]
    Srgb,

    #[display("srgb-linear")]
    SrgbLinear,

    #[display("display-p3")]
    DisplayP3,

    #[display("a98-rgb")]
    A98RGB,

    #[display("prophoto-rgb")]
    ProPhotoRGB,

    #[display("rec2020")]
    Rec2020,

    #[display("xyz")]
    XYZ,

    #[display("xyz-d50")]
    XYZD50,

    #[display("xyz-d65")]
    XYZD65,
}

#[derive(Clone, Copy, Debug, PartialEq, From, Serialize, Deserialize)]
pub struct Color {
    space: ColorSpace,
    c1: Number,
    c2: Number,
    c3: Number,
    alpha: Number,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.alpha > Number::from(0) {
            write!(
                f,
                "color({} {} {} {} / {})",
                self.space, self.c1, self.c2, self.c3, self.alpha
            )
        } else {
            write!(
                f,
                "color({} {} {} {})",
                self.space, self.c1, self.c2, self.c3
            )
        }
    }
}

impl FromStr for Color {
    type Err = anyhow::Error;

    /// Parses a string into a `Variable` value.
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Handle hex colors
        if s.starts_with("#") {
            return parse_hex_color(s);
        }

        // Handle functional notation (color())
        let chunks = s
            .replace("color(", "")
            .replace(")", "")
            .trim()
            .split(" ")
            .into_iter()
            .filter(|s| !s.is_empty() && *s != "/")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        if chunks.len() < 3 || chunks.len() > 5 {
            return Err(anyhow::anyhow!("Invalid color format"));
        }

        let space = chunks[0].parse::<ColorSpace>()?;
        let c1 = chunks[1].parse::<Number>()?;
        let c2 = chunks[2].parse::<Number>()?;
        let c3 = chunks[3].parse::<Number>()?;

        let alpha = chunks.get(4).map(|s| s.replace("%", "").parse::<Number>()).unwrap_or(Ok(Number::from(100)))?;

        Ok(Color {
            space,
            c1,
            c2,
            c3,
            alpha,
        })
    }
}

/// Parse a hex color string into a Color
fn parse_hex_color(hex: &str) -> Result<Color, anyhow::Error> {
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
        },
        4 => {
            // #RGBA format
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)? as i32;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)? as i32;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)? as i32;
            let a = u8::from_str_radix(&hex[3..4].repeat(2), 16)? as i32;
            (r, g, b, a)
        },
        6 => {
            // #RRGGBB format
            let r = u8::from_str_radix(&hex[0..2], 16)? as i32;
            let g = u8::from_str_radix(&hex[2..4], 16)? as i32;
            let b = u8::from_str_radix(&hex[4..6], 16)? as i32;
            (r, g, b, 255)
        },
        8 => {
            // #RRGGBBAA format
            let r = u8::from_str_radix(&hex[0..2], 16)? as i32;
            let g = u8::from_str_radix(&hex[2..4], 16)? as i32;
            let b = u8::from_str_radix(&hex[4..6], 16)? as i32;
            let a = u8::from_str_radix(&hex[6..8], 16)? as i32;
            (r, g, b, a)
        },
        _ => return Err(anyhow::anyhow!("Invalid hex color format"))
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
            "#F00".parse::<Color>().unwrap(),
            "#F00A".parse::<Color>().unwrap(),
            "#00FF00".parse::<Color>().unwrap(),
            "#0000FF80".parse::<Color>().unwrap(),
        ]);
    }

    #[test]
    fn test_parse_invalid_hex_colors() {
        goldie::assert_json!(vec![
            "#1".parse::<Color>().unwrap_err(),
            "#12345".parse::<Color>().unwrap_err(),
            "#GHIJKL".parse::<Color>().unwrap_err(),
        ].iter().map(|e| e.to_string()).collect::<Vec<_>>());
    }

    #[test]
    fn test_parse_functional_colors() {
        goldie::assert_json!(vec![
            "color(srgb 100 50 25)".parse::<Color>().unwrap(),
            "color(srgb 100 50 25 / 75)".parse::<Color>().unwrap(),
        ]);
    }

    #[test]
    fn test_parse_invalid_functional_colors() {
        goldie::assert_json!(vec![
            "color()".parse::<Color>().unwrap_err(),
            "color(invalid 100 50 25)".parse::<Color>().unwrap_err(),
            "color(srgb)".parse::<Color>().unwrap_err(),
            "color(srgb 100)".parse::<Color>().unwrap_err(),
        ].iter().map(|e| e.to_string()).collect::<Vec<_>>());
    }

    #[test]
    fn test_color_display() {
        goldie::assert_json!(vec![
          Color {
              space: ColorSpace::Srgb,
              c1: Number::from(100),
              c2: Number::from(50),
              c3: Number::from(25),
              alpha: Number::from(0),
          },
          Color {
              space: ColorSpace::Srgb,
              c1: Number::from(100),
              c2: Number::from(50),
              c3: Number::from(25),
              alpha: Number::from(75),
          },
          Color {
              space: ColorSpace::DisplayP3,
              c1: Number::from(80),
              c2: Number::from(90),
              c3: Number::from(100),
              alpha: Number::from(100),
          },
        ].iter().map(|c| c.to_string()).collect::<Vec<_>>());
    }
}
