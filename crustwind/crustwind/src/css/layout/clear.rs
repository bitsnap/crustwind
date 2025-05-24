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
pub enum Clear {
    #[display("left")]
    Left,

    #[display("right")]
    Right,

    #[display("both")]
    Both,

    #[display("none")]
    None,
}

impl FromStr for Clear {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(Clear::Left),
            "right" => Ok(Clear::Right),
            "both" => Ok(Clear::Both),
            "none" => Ok(Clear::None),
            _ => Err(anyhow::anyhow!("Invalid clear value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_display() {
        goldie::assert_json!(
            vec![Clear::Left, Clear::Right, Clear::Both, Clear::None,]
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_clear_from_str() {
        assert_eq!(Clear::from_str("left").unwrap(), Clear::Left);
        assert_eq!(Clear::from_str("right").unwrap(), Clear::Right);
        assert_eq!(Clear::from_str("both").unwrap(), Clear::Both);
        assert_eq!(Clear::from_str("none").unwrap(), Clear::None);
    }
}
