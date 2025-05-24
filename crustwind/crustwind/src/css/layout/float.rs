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
pub enum Float {
    #[display("left")]
    Left,

    #[display("right")]
    Right,

    #[display("none")]
    None,
}

impl FromStr for Float {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(Float::Left),
            "right" => Ok(Float::Right),
            "none" => Ok(Float::None),
            _ => Err(anyhow::anyhow!("Invalid float value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_float_display() {
        goldie::assert_json!(
            vec![Float::Left, Float::Right, Float::None,]
                .iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_float_from_str() {
        assert_eq!(Float::from_str("left").unwrap(), Float::Left);
        assert_eq!(Float::from_str("right").unwrap(), Float::Right);
        assert_eq!(Float::from_str("none").unwrap(), Float::None);
    }
}
