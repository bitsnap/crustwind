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

mod bottom;
mod left;
mod right;
mod top;

pub use bottom::*;
pub use left::*;
pub use right::*;
pub use top::*;

use derive_more::*;
use serde::*;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, From, Display, Serialize, Deserialize)]
pub enum Position {
    #[display("static")]
    Static,

    #[display("fixed")]
    Fixed,

    #[display("absolute")]
    Absolute,

    #[display("relative")]
    Relative,

    #[display("sticky")]
    Sticky,
}

impl FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "static" => Ok(Position::Static),
            "fixed" => Ok(Position::Fixed),
            "absolute" => Ok(Position::Absolute),
            "relative" => Ok(Position::Relative),
            "sticky" => Ok(Position::Sticky),
            _ => Err(anyhow::anyhow!("Invalid position value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_display() {
        goldie::assert_json!(
            vec![
                Position::Static,
                Position::Fixed,
                Position::Absolute,
                Position::Relative,
                Position::Sticky,
            ]
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_position_from_str() {
        assert_eq!(Position::from_str("static").unwrap(), Position::Static);
        assert_eq!(Position::from_str("fixed").unwrap(), Position::Fixed);
        assert_eq!(Position::from_str("absolute").unwrap(), Position::Absolute);
        assert_eq!(Position::from_str("relative").unwrap(), Position::Relative);
        assert_eq!(Position::from_str("sticky").unwrap(), Position::Sticky);
    }
}
