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
pub struct Gap(pub LengthUnit);

impl FromStr for Gap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        LengthUnit::from_str(s)
            .map(Gap)
            .map_err(|_| anyhow::anyhow!("Invalid gap value: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::units::{Number, Pct, Px};

    #[test]
    fn test_gap_display() {
        goldie::assert_json!(
            vec![
                Gap(LengthUnit::Px(Px(10.0))),
                Gap(LengthUnit::Pct(Pct(50.0))),
                Gap(LengthUnit::Number(Number(0.0))),
            ]
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_gap_from_str() {
        assert_eq!(
            Gap::from_str("10px").unwrap(),
            Gap(LengthUnit::Px(Px(10.0)))
        );
        assert_eq!(
            Gap::from_str("50%").unwrap(),
            Gap(LengthUnit::Pct(Pct(50.0)))
        );
        assert_eq!(
            Gap::from_str("0").unwrap(),
            Gap(LengthUnit::Number(Number(0.0)))
        );
        assert!(Gap::from_str("invalid").is_err());
    }
}
