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
pub enum BreakBefore {
    #[display("auto")]
    Auto,

    #[display("avoid")]
    Avoid,

    #[display("all")]
    All,

    #[display("avoid-page")]
    AvoidPage,

    #[display("page")]
    Page,

    #[display("left")]
    Left,

    #[display("right")]
    Right,

    #[display("column")]
    Column,
}

impl FromStr for BreakBefore {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(BreakBefore::Auto),
            "avoid" => Ok(BreakBefore::Avoid),
            "all" => Ok(BreakBefore::All),
            "avoid-page" => Ok(BreakBefore::AvoidPage),
            "page" => Ok(BreakBefore::Page),
            "left" => Ok(BreakBefore::Left),
            "right" => Ok(BreakBefore::Right),
            "column" => Ok(BreakBefore::Column),
            _ => Err(anyhow::anyhow!("Invalid break-before value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_before_display() {
        goldie::assert_json!(
            vec![
                BreakBefore::Auto,
                BreakBefore::Avoid,
                BreakBefore::All,
                BreakBefore::AvoidPage,
                BreakBefore::Page,
                BreakBefore::Left,
                BreakBefore::Right,
                BreakBefore::Column,
            ]
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_break_before_from_str() {
        assert_eq!(BreakBefore::from_str("auto").unwrap(), BreakBefore::Auto);
        assert_eq!(BreakBefore::from_str("avoid").unwrap(), BreakBefore::Avoid);
        assert_eq!(BreakBefore::from_str("all").unwrap(), BreakBefore::All);
        assert_eq!(
            BreakBefore::from_str("avoid-page").unwrap(),
            BreakBefore::AvoidPage
        );
        assert_eq!(BreakBefore::from_str("page").unwrap(), BreakBefore::Page);
        assert_eq!(BreakBefore::from_str("left").unwrap(), BreakBefore::Left);
        assert_eq!(BreakBefore::from_str("right").unwrap(), BreakBefore::Right);
        assert_eq!(
            BreakBefore::from_str("column").unwrap(),
            BreakBefore::Column
        );
    }
}
