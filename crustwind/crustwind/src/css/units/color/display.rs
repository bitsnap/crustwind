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

pub(crate) fn parse(s: &str) -> Result<Color, anyhow::Error> {
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

    let alpha = chunks
        .get(4)
        .map(|s| s.replace("%", "").parse::<Number>())
        .unwrap_or(Ok(Number::from(100)))?;

    Ok(Color {
        space,
        c1,
        c2,
        c3,
        alpha,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_functional_colors() {
        goldie::assert_json!(vec![
            parse("color(srgb 100 50 25)").unwrap(),
            parse("color(srgb 100 50 25 / 75)").unwrap(),
        ]);
    }

    #[test]
    fn test_parse_invalid_functional_colors() {
        goldie::assert_json!(
            vec![
                parse("color()").unwrap_err(),
                parse("color(invalid 100 50 25)").unwrap_err(),
                parse("color(srgb)").unwrap_err(),
                parse("color(srgb 100)").unwrap_err(),
            ]
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
        );
    }
}
