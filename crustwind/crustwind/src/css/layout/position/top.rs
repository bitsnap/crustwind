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

use crate::css::units::LengthUnit;

#[derive(Debug, Clone, Copy, PartialEq, From, Serialize, Deserialize)]
pub enum Top {
    #[display("auto")]
    Auto,
    #[display("{_0}")]
    Length(Length),
}

impl FromStr for Top {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "auto" {
            return Ok(Top::Auto);
        }

        match LengthUnit::from_str(s) {
            Ok(length) => Ok(Top::Length(length)),
            Err(_) => Err(anyhow::anyhow!("Invalid top value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::units::Unit;

    #[test]
    fn test_top_display() {
        goldie::assert_json!(
            vec![
                Top::Auto,
                Top::Length(Length::new(0.0, Unit::Px)),
                Top::Length(Length::new(10.0, Unit::Px)),
                Top::Length(Length::new(20.0, Unit::Percent)),
            ]
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_top_from_str() {
        assert_eq!(Top::from_str("auto").unwrap(), Top::Auto);
        assert_eq!(
            Top::from_str("0px").unwrap(),
            Top::Length(Length::new(0.0, Unit::Px))
        );
        assert_eq!(
            Top::from_str("10px").unwrap(),
            Top::Length(Length::new(10.0, Unit::Px))
        );
        assert_eq!(
            Top::from_str("20%").unwrap(),
            Top::Length(Length::new(20.0, Unit::Percent))
        );
    }
}
