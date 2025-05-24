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

#[derive(Debug, Clone, Copy, PartialEq, From, Display, Serialize, Deserialize)]
pub struct FlexShrink(pub f32);

impl FromStr for FlexShrink {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f32>() {
            Ok(value) if value >= 0.0 => Ok(FlexShrink(value)),
            _ => Err(anyhow::anyhow!("Invalid flex-shrink value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_shrink_display() {
        goldie::assert_json!(
            vec![FlexShrink(0.0), FlexShrink(1.0), FlexShrink(2.5)]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_flex_shrink_from_str() {
        assert_eq!(FlexShrink::from_str("0").unwrap(), FlexShrink(0.0));
        assert_eq!(FlexShrink::from_str("1").unwrap(), FlexShrink(1.0));
        assert_eq!(FlexShrink::from_str("2.5").unwrap(), FlexShrink(2.5));
        assert!(FlexShrink::from_str("-1").is_err());
        assert!(FlexShrink::from_str("invalid").is_err());
    }
}
