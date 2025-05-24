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
pub enum Overflow {
    #[display("auto")]
    Auto,

    #[display("hidden")]
    Hidden,

    #[display("clip")]
    Clip,

    #[display("visible")]
    Visible,

    #[display("scroll")]
    Scroll,
}

impl FromStr for Overflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Overflow::Auto),
            "hidden" => Ok(Overflow::Hidden),
            "clip" => Ok(Overflow::Clip),
            "visible" => Ok(Overflow::Visible),
            "scroll" => Ok(Overflow::Scroll),
            _ => Err(anyhow::anyhow!("Invalid overflow value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow_display() {
        goldie::assert_json!(
            vec![
                Overflow::Auto,
                Overflow::Hidden,
                Overflow::Clip,
                Overflow::Visible,
                Overflow::Scroll,
            ]
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_overflow_from_str() {
        assert_eq!(Overflow::from_str("auto").unwrap(), Overflow::Auto);
        assert_eq!(Overflow::from_str("hidden").unwrap(), Overflow::Hidden);
        assert_eq!(Overflow::from_str("clip").unwrap(), Overflow::Clip);
        assert_eq!(Overflow::from_str("visible").unwrap(), Overflow::Visible);
        assert_eq!(Overflow::from_str("scroll").unwrap(), Overflow::Scroll);
    }
}
