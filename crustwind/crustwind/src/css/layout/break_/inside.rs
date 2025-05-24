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
pub enum BreakInside {
    #[display("auto")]
    Auto,

    #[display("avoid")]
    Avoid,

    #[display("avoid-page")]
    AvoidPage,

    #[display("avoid-column")]
    AvoidColumn,
}

impl FromStr for BreakInside {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(BreakInside::Auto),
            "avoid" => Ok(BreakInside::Avoid),
            "avoid-page" => Ok(BreakInside::AvoidPage),
            "avoid-column" => Ok(BreakInside::AvoidColumn),
            _ => Err(anyhow::anyhow!("Invalid break-inside value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_inside_display() {
        goldie::assert_json!(
            vec![
                BreakInside::Auto,
                BreakInside::Avoid,
                BreakInside::AvoidPage,
                BreakInside::AvoidColumn,
            ]
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_break_inside_from_str() {
        assert_eq!(BreakInside::from_str("auto").unwrap(), BreakInside::Auto);
        assert_eq!(BreakInside::from_str("avoid").unwrap(), BreakInside::Avoid);
        assert_eq!(
            BreakInside::from_str("avoid-page").unwrap(),
            BreakInside::AvoidPage
        );
        assert_eq!(
            BreakInside::from_str("avoid-column").unwrap(),
            BreakInside::AvoidColumn
        );
    }
}
