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

use crate::css::units::Length;

#[derive(Debug, Clone, Copy, PartialEq, From, Display, Serialize, Deserialize)]
pub enum Right {
    Auto,
    Length(Length),
}

impl std::fmt::Display for Right {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Right::Auto => write!(f, "auto"),
            Right::Length(length) => write!(f, "{}", length),
        }
    }
}

impl FromStr for Right {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "auto" {
            return Ok(Right::Auto);
        }

        match Length::from_str(s) {
            Ok(length) => Ok(Right::Length(length)),
            Err(_) => Err(anyhow::anyhow!("Invalid right value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::units::Unit;

    #[test]
    fn test_right_display() {
        goldie::assert_json!(
            vec![
                Right::Auto,
                Right::Length(Length::new(0.0, Unit::Px)),
                Right::Length(Length::new(10.0, Unit::Px)),
                Right::Length(Length::new(20.0, Unit::Percent)),
            ]
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_right_from_str() {
        assert_eq!(Right::from_str("auto").unwrap(), Right::Auto);
        assert_eq!(
            Right::from_str("0px").unwrap(),
            Right::Length(Length::new(0.0, Unit::Px))
        );
        assert_eq!(
            Right::from_str("10px").unwrap(),
            Right::Length(Length::new(10.0, Unit::Px))
        );
        assert_eq!(
            Right::from_str("20%").unwrap(),
            Right::Length(Length::new(20.0, Unit::Percent))
        );
    }
}
