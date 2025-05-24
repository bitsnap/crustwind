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
pub enum Overscroll {
    #[display("auto")]
    Auto,

    #[display("contain")]
    Contain,

    #[display("none")]
    None,
}

impl FromStr for Overscroll {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Overscroll::Auto),
            "contain" => Ok(Overscroll::Contain),
            "none" => Ok(Overscroll::None),
            _ => Err(anyhow::anyhow!("Invalid overscroll value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overscroll_display() {
        goldie::assert_json!(
            vec![Overscroll::Auto, Overscroll::Contain, Overscroll::None,]
                .iter()
                .map(|o| o.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_overscroll_from_str() {
        assert_eq!(Overscroll::from_str("auto").unwrap(), Overscroll::Auto);
        assert_eq!(
            Overscroll::from_str("contain").unwrap(),
            Overscroll::Contain
        );
        assert_eq!(Overscroll::from_str("none").unwrap(), Overscroll::None);
    }
}
