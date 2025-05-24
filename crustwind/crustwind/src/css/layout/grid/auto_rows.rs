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

use crate::css::units::LengthUnit;
use derive_more::*;
use serde::*;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, From, Display, Serialize, Deserialize)]
pub enum GridAutoRows {
    #[display("auto")]
    Auto,

    #[display("min-content")]
    MinContent,

    #[display("max-content")]
    MaxContent,

    #[display("{0}")]
    Length(LengthUnit),
}

impl FromStr for GridAutoRows {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(GridAutoRows::Auto),
            "min-content" => Ok(GridAutoRows::MinContent),
            "max-content" => Ok(GridAutoRows::MaxContent),
            _ => LengthUnit::from_str(s)
                .map(GridAutoRows::Length)
                .map_err(|_| anyhow::anyhow!("Invalid grid-auto-rows value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::units::{Number, Pct, Px};

    #[test]
    fn test_grid_auto_rows_display() {
        goldie::assert_json!(
            vec![
                GridAutoRows::Auto,
                GridAutoRows::MinContent,
                GridAutoRows::MaxContent,
                GridAutoRows::Length(LengthUnit::Px(Px(10.0))),
                GridAutoRows::Length(LengthUnit::Pct(Pct(50.0))),
            ]
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_grid_auto_rows_from_str() {
        assert_eq!(GridAutoRows::from_str("auto").unwrap(), GridAutoRows::Auto);
        assert_eq!(GridAutoRows::from_str("min-content").unwrap(), GridAutoRows::MinContent);
        assert_eq!(GridAutoRows::from_str("max-content").unwrap(), GridAutoRows::MaxContent);
        assert_eq!(
            GridAutoRows::from_str("10px").unwrap(),
            GridAutoRows::Length(LengthUnit::Px(Px(10.0)))
        );
        assert_eq!(
            GridAutoRows::from_str("50%").unwrap(),
            GridAutoRows::Length(LengthUnit::Pct(Pct(50.0)))
        );
        assert!(GridAutoRows::from_str("invalid").is_err());
    }
}
