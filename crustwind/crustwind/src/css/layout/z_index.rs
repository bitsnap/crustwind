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
pub enum ZIndex {
    #[display("auto")]
    Auto,
    #[display("{_0}")]
    Value(i32),
}

impl FromStr for ZIndex {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "auto" {
            return Ok(ZIndex::Auto);
        }

        match s.parse::<i32>() {
            Ok(val) => Ok(ZIndex::Value(val)),
            Err(_) => Err(anyhow::anyhow!("Invalid z-index value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_index_display() {
        goldie::assert_json!(
            vec![
                ZIndex::Auto,
                ZIndex::Value(0),
                ZIndex::Value(10),
                ZIndex::Value(20),
                ZIndex::Value(30),
                ZIndex::Value(40),
                ZIndex::Value(50),
                ZIndex::Value(-10),
            ]
            .iter()
            .map(|z| z.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_z_index_from_str() {
        assert_eq!(ZIndex::from_str("auto").unwrap(), ZIndex::Auto);
        assert_eq!(ZIndex::from_str("0").unwrap(), ZIndex::Value(0));
        assert_eq!(ZIndex::from_str("10").unwrap(), ZIndex::Value(10));
        assert_eq!(ZIndex::from_str("-10").unwrap(), ZIndex::Value(-10));
    }
}
