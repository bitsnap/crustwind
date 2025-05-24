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
pub struct GridColumn {
    pub start: GridColumnValue,
    pub end: Option<GridColumnValue>,
}

#[derive(Debug, Clone, PartialEq, From, Display, Serialize, Deserialize)]
pub enum GridColumnValue {
    #[display("auto")]
    Auto,

    #[display("{0}")]
    Number(i32),

    #[display("span {0}")]
    Span(i32),
}

impl FromStr for GridColumn {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("/") {
            let parts: Vec<&str> = s.split("/").collect();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!("Invalid grid-column value: {}", s));
            }
            
            let start = GridColumnValue::from_str(parts[0].trim())?;
            let end = GridColumnValue::from_str(parts[1].trim())?;
            
            Ok(GridColumn {
                start,
                end: Some(end),
            })
        } else {
            let start = GridColumnValue::from_str(s)?;
            Ok(GridColumn {
                start,
                end: None,
            })
        }
    }
}

impl FromStr for GridColumnValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "auto" {
            return Ok(GridColumnValue::Auto);
        }
        
        if s.starts_with("span ") {
            let span_value = s.trim_start_matches("span ").parse::<i32>()
                .map_err(|_| anyhow::anyhow!("Invalid grid-column span value: {}", s))?;
            return Ok(GridColumnValue::Span(span_value));
        }
        
        s.parse::<i32>()
            .map(GridColumnValue::Number)
            .map_err(|_| anyhow::anyhow!("Invalid grid-column value: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_column_display() {
        goldie::assert_json!(
            vec![
                GridColumn {
                    start: GridColumnValue::Auto,
                    end: None,
                },
                GridColumn {
                    start: GridColumnValue::Number(1),
                    end: Some(GridColumnValue::Number(3)),
                },
                GridColumn {
                    start: GridColumnValue::Span(2),
                    end: None,
                },
            ]
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_grid_column_from_str() {
        assert_eq!(
            GridColumn::from_str("auto").unwrap(),
            GridColumn {
                start: GridColumnValue::Auto,
                end: None,
            }
        );
        assert_eq!(
            GridColumn::from_str("1 / 3").unwrap(),
            GridColumn {
                start: GridColumnValue::Number(1),
                end: Some(GridColumnValue::Number(3)),
            }
        );
        assert_eq!(
            GridColumn::from_str("span 2").unwrap(),
            GridColumn {
                start: GridColumnValue::Span(2),
                end: None,
            }
        );
        assert!(GridColumn::from_str("invalid").is_err());
    }
}
