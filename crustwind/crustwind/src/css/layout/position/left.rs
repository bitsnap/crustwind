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
pub enum Left {
    Auto,
    Length(Length),
}

impl std::fmt::Display for Left {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Left::Auto => write!(f, "auto"),
            Left::Length(length) => write!(f, "{}", length),
        }
    }
}

impl FromStr for Left {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "auto" {
            return Ok(Left::Auto);
        }

        match Length::from_str(s) {
            Ok(length) => Ok(Left::Length(length)),
            Err(_) => Err(anyhow::anyhow!("Invalid left value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::units::Unit;

    #[test]
    fn test_left_display() {
        goldie::assert_json!(
            vec![
                Left::Auto,
                Left::Length(Length::new(0.0, Unit::Px)),
                Left::Length(Length::new(10.0, Unit::Px)),
                Left::Length(Length::new(20.0, Unit::Percent)),
            ]
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_left_from_str() {
        assert_eq!(Left::from_str("auto").unwrap(), Left::Auto);
        assert_eq!(
            Left::from_str("0px").unwrap(),
            Left::Length(Length::new(0.0, Unit::Px))
        );
        assert_eq!(
            Left::from_str("10px").unwrap(),
            Left::Length(Length::new(10.0, Unit::Px))
        );
        assert_eq!(
            Left::from_str("20%").unwrap(),
            Left::Length(Length::new(20.0, Unit::Percent))
        );
    }
}
