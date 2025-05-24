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
pub enum GridAutoFlow {
    #[display("row")]
    Row,

    #[display("column")]
    Column,

    #[display("row dense")]
    RowDense,

    #[display("column dense")]
    ColumnDense,
}

impl FromStr for GridAutoFlow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "row" => Ok(GridAutoFlow::Row),
            "column" => Ok(GridAutoFlow::Column),
            "row dense" => Ok(GridAutoFlow::RowDense),
            "column dense" => Ok(GridAutoFlow::ColumnDense),
            _ => Err(anyhow::anyhow!("Invalid grid-auto-flow value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_auto_flow_display() {
        goldie::assert_json!(
            vec![
                GridAutoFlow::Row,
                GridAutoFlow::Column,
                GridAutoFlow::RowDense,
                GridAutoFlow::ColumnDense,
            ]
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_grid_auto_flow_from_str() {
        assert_eq!(GridAutoFlow::from_str("row").unwrap(), GridAutoFlow::Row);
        assert_eq!(GridAutoFlow::from_str("column").unwrap(), GridAutoFlow::Column);
        assert_eq!(GridAutoFlow::from_str("row dense").unwrap(), GridAutoFlow::RowDense);
        assert_eq!(GridAutoFlow::from_str("column dense").unwrap(), GridAutoFlow::ColumnDense);
        assert!(GridAutoFlow::from_str("invalid").is_err());
    }
}
