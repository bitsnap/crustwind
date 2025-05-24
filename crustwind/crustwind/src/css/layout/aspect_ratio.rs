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
use serde::*;
use std::str::FromStr;

use crate::css::units::*;

#[derive(Debug, Clone, PartialEq, From, Display, Serialize, Deserialize)]
pub enum AspectRatio {
    #[display("auto")]
    Auto,

    #[display("square")]
    Square,

    #[display("video")]
    Video,

    #[display("{0}/{1}")]
    Ratio(u32, u32),

    #[display("0")]
    Variable(Variable),
}

impl FromStr for AspectRatio {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(AspectRatio::Auto),
            "square" => Ok(AspectRatio::Square),
            "video" => Ok(AspectRatio::Video),
            _ => {
                if s.starts_with("var(") && s.ends_with(")") {
                    let var_name = s[4..s.len() - 1].to_string();
                    return Ok(AspectRatio::Variable(
                        css::units::variable::Variable::from_str(var_name)?,
                    ));
                }

                if let Some(idx) = s.find('/') {
                    let (width, height) = s.split_at(idx);
                    let height = &height[1..]; // Skip the '/' character

                    let width_val = width.trim().parse::<u32>()?;
                    let height_val = height.trim().parse::<u32>()?;

                    return Ok(AspectRatio::Ratio(width_val, height_val));
                }

                Err(anyhow::anyhow!("Invalid aspect ratio format"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspect_ratio_display() {
        goldie::assert_json!(
            vec![
                AspectRatio::Auto,
                AspectRatio::Square,
                AspectRatio::Video,
                AspectRatio::Ratio(3, 2),
                AspectRatio::Variable("--my-aspect-ratio".to_string()),
            ]
            .iter()
            .map(|ar| ar.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_aspect_ratio_from_str() {
        assert_eq!(AspectRatio::from_str("auto").unwrap(), AspectRatio::Auto);
        assert_eq!(
            AspectRatio::from_str("square").unwrap(),
            AspectRatio::Square
        );
        assert_eq!(AspectRatio::from_str("video").unwrap(), AspectRatio::Video);
        assert_eq!(
            AspectRatio::from_str("3/2").unwrap(),
            AspectRatio::Ratio(3, 2)
        );
        assert_eq!(
            AspectRatio::from_str("var(--my-aspect-ratio)").unwrap(),
            AspectRatio::Variable("--my-aspect-ratio".to_string())
        );
    }
}
