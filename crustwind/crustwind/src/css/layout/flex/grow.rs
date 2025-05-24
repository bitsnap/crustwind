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
pub struct FlexGrow(pub f32);

impl FromStr for FlexGrow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f32>() {
            Ok(value) if value >= 0.0 => Ok(FlexGrow(value)),
            _ => Err(anyhow::anyhow!("Invalid flex-grow value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_grow_display() {
        goldie::assert_json!(
            vec![FlexGrow(0.0), FlexGrow(1.0), FlexGrow(2.5)]
                .iter()
                .map(|g| g.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_flex_grow_from_str() {
        assert_eq!(FlexGrow::from_str("0").unwrap(), FlexGrow(0.0));
        assert_eq!(FlexGrow::from_str("1").unwrap(), FlexGrow(1.0));
        assert_eq!(FlexGrow::from_str("2.5").unwrap(), FlexGrow(2.5));
        assert!(FlexGrow::from_str("-1").is_err());
        assert!(FlexGrow::from_str("invalid").is_err());
    }
}
