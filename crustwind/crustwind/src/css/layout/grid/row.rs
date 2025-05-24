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

#[derive(Debug, Clone, PartialEq, From, Display, Serialize, Deserialize)]
pub struct GridRow {
    pub start: GridRowValue,
    pub end: Option<GridRowValue>,
}

#[derive(Debug, Clone, PartialEq, From, Display, Serialize, Deserialize)]
pub enum GridRowValue {
    #[display("auto")]
    Auto,

    #[display("{0}")]
    Number(i32),

    #[display("span {0}")]
    Span(i32),
}

impl FromStr for GridRow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("/") {
            let parts: Vec<&str> = s.split("/").collect();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!("Invalid grid-row value: {}", s));
            }
            
            let start = GridRowValue::from_str(parts[0].trim())?;
            let end = GridRowValue::from_str(parts[1].trim())?;
            
            Ok(GridRow {
                start,
                end: Some(end),
            })
        } else {
            let start = GridRowValue::from_str(s)?;
            Ok(GridRow {
                start,
                end: None,
            })
        }
    }
}

impl FromStr for GridRowValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "auto" {
            return Ok(GridRowValue::Auto);
        }
        
        if s.starts_with("span ") {
            let span_value = s.trim_start_matches("span ").parse::<i32>()
                .map_err(|_| anyhow::anyhow!("Invalid grid-row span value: {}", s))?;
            return Ok(GridRowValue::Span(span_value));
        }
        
        s.parse::<i32>()
            .map(GridRowValue::Number)
            .map_err(|_| anyhow::anyhow!("Invalid grid-row value: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_row_display() {
        goldie::assert_json!(
            vec![
                GridRow {
                    start: GridRowValue::Auto,
                    end: None,
                },
                GridRow {
                    start: GridRowValue::Number(1),
                    end: Some(GridRowValue::Number(3)),
                },
                GridRow {
                    start: GridRowValue::Span(2),
                    end: None,
                },
            ]
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_grid_row_from_str() {
        assert_eq!(
            GridRow::from_str("auto").unwrap(),
            GridRow {
                start: GridRowValue::Auto,
                end: None,
            }
        );
        assert_eq!(
            GridRow::from_str("1 / 3").unwrap(),
            GridRow {
                start: GridRowValue::Number(1),
                end: Some(GridRowValue::Number(3)),
            }
        );
        assert_eq!(
            GridRow::from_str("span 2").unwrap(),
            GridRow {
                start: GridRowValue::Span(2),
                end: None,
            }
        );
        assert!(GridRow::from_str("invalid").is_err());
    }
}
