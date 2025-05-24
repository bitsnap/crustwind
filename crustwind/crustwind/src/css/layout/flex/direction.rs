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
pub enum FlexDirection {
    #[display("row")]
    Row,

    #[display("row-reverse")]
    RowReverse,

    #[display("column")]
    Column,

    #[display("column-reverse")]
    ColumnReverse,
}

impl FromStr for FlexDirection {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "row" => Ok(FlexDirection::Row),
            "row-reverse" => Ok(FlexDirection::RowReverse),
            "column" => Ok(FlexDirection::Column),
            "column-reverse" => Ok(FlexDirection::ColumnReverse),
            _ => Err(anyhow::anyhow!("Invalid flex-direction value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_direction_display() {
        goldie::assert_json!(
            vec![
                FlexDirection::Row,
                FlexDirection::RowReverse,
                FlexDirection::Column,
                FlexDirection::ColumnReverse,
            ]
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_flex_direction_from_str() {
        assert_eq!(FlexDirection::from_str("row").unwrap(), FlexDirection::Row);
        assert_eq!(FlexDirection::from_str("row-reverse").unwrap(), FlexDirection::RowReverse);
        assert_eq!(FlexDirection::from_str("column").unwrap(), FlexDirection::Column);
        assert_eq!(FlexDirection::from_str("column-reverse").unwrap(), FlexDirection::ColumnReverse);
    }
}
