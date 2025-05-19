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

mod display;
mod hex;

use derive_more::*;
use serde::*;

use crate::css::units::Number;

#[derive(
    derive_more::Debug, Clone, Copy, PartialEq, Display, From, FromStr, Serialize, Deserialize,
)]
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

#[derive(derive_more::Debug, Clone, Copy, PartialEq, From, Serialize, Deserialize)]
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

impl std::str::FromStr for Color {
    type Err = anyhow::Error;

    /// Parses a string into a `Variable` value.
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Handle hex colors
        if s.starts_with("#") {
            return hex::parse(s);
        }

        if s.starts_with("color(") {
            return display::parse(s);
        }

        Err(anyhow::anyhow!("Invalid color format"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_display() {
        goldie::assert_json!(
            vec![
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
            ]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
        );
    }
}
