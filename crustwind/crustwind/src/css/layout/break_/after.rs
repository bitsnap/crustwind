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
pub enum BreakAfter {
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

impl FromStr for BreakAfter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(BreakAfter::Auto),
            "avoid" => Ok(BreakAfter::Avoid),
            "all" => Ok(BreakAfter::All),
            "avoid-page" => Ok(BreakAfter::AvoidPage),
            "page" => Ok(BreakAfter::Page),
            "left" => Ok(BreakAfter::Left),
            "right" => Ok(BreakAfter::Right),
            "column" => Ok(BreakAfter::Column),
            _ => Err(anyhow::anyhow!("Invalid break-after value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_after_display() {
        goldie::assert_json!(
            vec![
                BreakAfter::Auto,
                BreakAfter::Avoid,
                BreakAfter::All,
                BreakAfter::AvoidPage,
                BreakAfter::Page,
                BreakAfter::Left,
                BreakAfter::Right,
                BreakAfter::Column,
            ]
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_break_after_from_str() {
        assert_eq!(BreakAfter::from_str("auto").unwrap(), BreakAfter::Auto);
        assert_eq!(BreakAfter::from_str("avoid").unwrap(), BreakAfter::Avoid);
        assert_eq!(BreakAfter::from_str("all").unwrap(), BreakAfter::All);
        assert_eq!(
            BreakAfter::from_str("avoid-page").unwrap(),
            BreakAfter::AvoidPage
        );
        assert_eq!(BreakAfter::from_str("page").unwrap(), BreakAfter::Page);
        assert_eq!(BreakAfter::from_str("left").unwrap(), BreakAfter::Left);
        assert_eq!(BreakAfter::from_str("right").unwrap(), BreakAfter::Right);
        assert_eq!(BreakAfter::from_str("column").unwrap(), BreakAfter::Column);
    }
}
