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
pub enum Columns {
    Auto,
    Count(u32),
}

impl std::fmt::Display for Columns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Columns::Auto => write!(f, "auto"),
            Columns::Count(count) => write!(f, "{}", count),
        }
    }
}

impl FromStr for Columns {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "auto" {
            return Ok(Columns::Auto);
        }

        match s.parse::<u32>() {
            Ok(count) => Ok(Columns::Count(count)),
            Err(_) => Err(anyhow::anyhow!("Invalid columns value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_columns_display() {
        goldie::assert_json!(
            vec![
                Columns::Auto,
                Columns::Count(1),
                Columns::Count(2),
                Columns::Count(3),
                Columns::Count(4),
            ]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_columns_from_str() {
        assert_eq!(Columns::from_str("auto").unwrap(), Columns::Auto);
        assert_eq!(Columns::from_str("1").unwrap(), Columns::Count(1));
        assert_eq!(Columns::from_str("2").unwrap(), Columns::Count(2));
        assert_eq!(Columns::from_str("3").unwrap(), Columns::Count(3));
    }
}
