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
pub enum FlexBasis {
    #[display("auto")]
    Auto,

    #[display("{0}")]
    Length(LengthUnit),
}

impl FromStr for FlexBasis {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "auto" {
            return Ok(FlexBasis::Auto);
        }
        
        LengthUnit::from_str(s)
            .map(FlexBasis::Length)
            .map_err(|_| anyhow::anyhow!("Invalid flex-basis value: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::units::{Number, Pct, Px};

    #[test]
    fn test_flex_basis_display() {
        goldie::assert_json!(
            vec![
                FlexBasis::Auto,
                FlexBasis::Length(LengthUnit::Px(Px(10.0))),
                FlexBasis::Length(LengthUnit::Pct(Pct(50.0))),
            ]
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_flex_basis_from_str() {
        assert_eq!(FlexBasis::from_str("auto").unwrap(), FlexBasis::Auto);
        assert_eq!(
            FlexBasis::from_str("10px").unwrap(),
            FlexBasis::Length(LengthUnit::Px(Px(10.0)))
        );
        assert_eq!(
            FlexBasis::from_str("50%").unwrap(),
            FlexBasis::Length(LengthUnit::Pct(Pct(50.0)))
        );
        assert_eq!(
            FlexBasis::from_str("0").unwrap(),
            FlexBasis::Length(LengthUnit::Number(Number(0.0)))
        );
        assert!(FlexBasis::from_str("invalid").is_err());
    }
}
