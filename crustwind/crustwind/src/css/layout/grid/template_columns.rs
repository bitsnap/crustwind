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

use crate::css::units::LengthUnit;
use derive_more::*;
use serde::*;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, From, Display, Serialize, Deserialize)]
pub enum GridTemplateColumns {
    #[display("none")]
    None,

    #[display("{0}")]
    Template(GridTemplate),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridTemplate(pub Vec<GridTemplateValue>);

impl std::fmt::Display for GridTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values: Vec<String> = self.0.iter().map(|v| v.to_string()).collect();
        write!(f, "{}", values.join(" "))
    }
}

#[derive(Debug, Clone, PartialEq, From, Display, Serialize, Deserialize)]
pub enum GridTemplateValue {
    #[display("auto")]
    Auto,

    #[display("min-content")]
    MinContent,

    #[display("max-content")]
    MaxContent,

    #[display("{0}fr")]
    Fr(f32),

    #[display("{0}")]
    Length(LengthUnit),
}

impl FromStr for GridTemplateColumns {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "none" {
            return Ok(GridTemplateColumns::None);
        }
        
        let template = GridTemplate::from_str(s)?;
        Ok(GridTemplateColumns::Template(template))
    }
}

impl FromStr for GridTemplate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let mut values = Vec::new();
        
        for part in parts {
            let value = GridTemplateValue::from_str(part)?;
            values.push(value);
        }
        
        if values.is_empty() {
            return Err(anyhow::anyhow!("Invalid grid template: {}", s));
        }
        
        Ok(GridTemplate(values))
    }
}

impl FromStr for GridTemplateValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(GridTemplateValue::Auto),
            "min-content" => Ok(GridTemplateValue::MinContent),
            "max-content" => Ok(GridTemplateValue::MaxContent),
            _ if s.ends_with("fr") => {
                let fr_value = s.trim_end_matches("fr").parse::<f32>()
                    .map_err(|_| anyhow::anyhow!("Invalid fr value: {}", s))?;
                Ok(GridTemplateValue::Fr(fr_value))
            },
            _ => LengthUnit::from_str(s)
                .map(GridTemplateValue::Length)
                .map_err(|_| anyhow::anyhow!("Invalid grid template value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css::units::{Pct, Px};

    #[test]
    fn test_grid_template_columns_display() {
        goldie::assert_json!(
            vec![
                GridTemplateColumns::None,
                GridTemplateColumns::Template(GridTemplate(vec![
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
    fn test_grid_template_columns_from_str() {
        assert_eq!(GridTemplateColumns::from_str("none").unwrap(), GridTemplateColumns::None);
        assert_eq!(
            GridTemplateColumns::from_str("auto 1fr 100px").unwrap(),
            GridTemplateColumns::Template(GridTemplate(vec![
                GridTemplateValue::Auto,
                GridTemplateValue::Fr(1.0),
                GridTemplateValue::Length(LengthUnit::Px(Px(100.0))),
            ]))
        );
        assert_eq!(
            GridTemplateColumns::from_str("1fr 2fr 50%").unwrap(),
            GridTemplateColumns::Template(GridTemplate(vec![
                GridTemplateValue::Fr(1.0),
                GridTemplateValue::Fr(2.0),
                GridTemplateValue::Length(LengthUnit::Pct(Pct(50.0))),
            ]))
        );
        assert!(GridTemplateColumns::from_str("").is_err());
    }
}
