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

use crate::css::layout::grid::template_columns::{GridTemplate, GridTemplateValue};
use derive_more::*;
use serde::*;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, From, Display, Serialize, Deserialize)]
pub enum GridTemplateRows {
    #[display("none")]
    None,

    #[display("{0}")]
    Template(GridTemplate),
}

impl FromStr for GridTemplateRows {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "none" {
            return Ok(GridTemplateRows::None);
        }
        
        let template = GridTemplate::from_str(s)?;
        Ok(GridTemplateRows::Template(template))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::units::{LengthUnit, Pct, Px};

    #[test]
    fn test_grid_template_rows_display() {
        goldie::assert_json!(
            vec![
                GridTemplateRows::None,
                GridTemplateRows::Template(GridTemplate(vec![
                    GridTemplateValue::Auto,
                    GridTemplateValue::Fr(1.0),
                    GridTemplateValue::Length(LengthUnit::Px(Px(100.0))),
                ])),
            ]
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_grid_template_rows_from_str() {
        assert_eq!(GridTemplateRows::from_str("none").unwrap(), GridTemplateRows::None);
        assert_eq!(
            GridTemplateRows::from_str("auto 1fr 100px").unwrap(),
            GridTemplateRows::Template(GridTemplate(vec![
                GridTemplateValue::Auto,
                GridTemplateValue::Fr(1.0),
                GridTemplateValue::Length(LengthUnit::Px(Px(100.0))),
            ]))
        );
        assert_eq!(
            GridTemplateRows::from_str("1fr 2fr 50%").unwrap(),
            GridTemplateRows::Template(GridTemplate(vec![
                GridTemplateValue::Fr(1.0),
                GridTemplateValue::Fr(2.0),
                GridTemplateValue::Length(LengthUnit::Pct(Pct(50.0))),
            ]))
        );
        assert!(GridTemplateRows::from_str("").is_err());
    }
}
