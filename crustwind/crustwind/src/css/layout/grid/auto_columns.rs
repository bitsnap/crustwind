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
pub enum GridAutoColumns {
    #[display("auto")]
    Auto,

    #[display("min-content")]
    MinContent,

    #[display("max-content")]
    MaxContent,

    #[display("{0}")]
    Length(LengthUnit),
}

impl FromStr for GridAutoColumns {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(GridAutoColumns::Auto),
            "min-content" => Ok(GridAutoColumns::MinContent),
            "max-content" => Ok(GridAutoColumns::MaxContent),
            _ => LengthUnit::from_str(s)
                .map(GridAutoColumns::Length)
                .map_err(|_| anyhow::anyhow!("Invalid grid-auto-columns value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::units::{Number, Pct, Px};

    #[test]
    fn test_grid_auto_columns_display() {
        goldie::assert_json!(
            vec![
                GridAutoColumns::Auto,
                GridAutoColumns::MinContent,
                GridAutoColumns::MaxContent,
                GridAutoColumns::Length(LengthUnit::Px(Px(10.0))),
                GridAutoColumns::Length(LengthUnit::Pct(Pct(50.0))),
            ]
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_grid_auto_columns_from_str() {
        assert_eq!(GridAutoColumns::from_str("auto").unwrap(), GridAutoColumns::Auto);
        assert_eq!(GridAutoColumns::from_str("min-content").unwrap(), GridAutoColumns::MinContent);
        assert_eq!(GridAutoColumns::from_str("max-content").unwrap(), GridAutoColumns::MaxContent);
        assert_eq!(
            GridAutoColumns::from_str("10px").unwrap(),
            GridAutoColumns::Length(LengthUnit::Px(Px(10.0)))
        );
        assert_eq!(
            GridAutoColumns::from_str("50%").unwrap(),
            GridAutoColumns::Length(LengthUnit::Pct(Pct(50.0)))
        );
        assert!(GridAutoColumns::from_str("invalid").is_err());
    }
}
