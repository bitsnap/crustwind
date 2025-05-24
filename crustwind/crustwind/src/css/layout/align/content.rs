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
pub enum AlignContent {
    #[display("center")]
    Center,

    #[display("start")]
    Start,

    #[display("end")]
    End,

    #[display("flex-start")]
    FlexStart,

    #[display("flex-end")]
    FlexEnd,

    #[display("normal")]
    Normal,

    #[display("space-between")]
    SpaceBetween,

    #[display("space-around")]
    SpaceAround,

    #[display("space-evenly")]
    SpaceEvenly,

    #[display("stretch")]
    Stretch,

    #[display("baseline")]
    Baseline,
}

impl FromStr for AlignContent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "center" => Ok(AlignContent::Center),
            "start" => Ok(AlignContent::Start),
            "end" => Ok(AlignContent::End),
            "flex-start" => Ok(AlignContent::FlexStart),
            "flex-end" => Ok(AlignContent::FlexEnd),
            "normal" => Ok(AlignContent::Normal),
            "space-between" => Ok(AlignContent::SpaceBetween),
            "space-around" => Ok(AlignContent::SpaceAround),
            "space-evenly" => Ok(AlignContent::SpaceEvenly),
            "stretch" => Ok(AlignContent::Stretch),
            "baseline" => Ok(AlignContent::Baseline),
            _ => Err(anyhow::anyhow!("Invalid align-content value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_content_display() {
        goldie::assert_json!(
            vec![
                AlignContent::Center,
                AlignContent::Start,
                AlignContent::End,
                AlignContent::FlexStart,
                AlignContent::FlexEnd,
                AlignContent::Normal,
                AlignContent::SpaceBetween,
                AlignContent::SpaceAround,
                AlignContent::SpaceEvenly,
                AlignContent::Stretch,
                AlignContent::Baseline,
            ]
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_align_content_from_str() {
        assert_eq!(AlignContent::from_str("center").unwrap(), AlignContent::Center);
        assert_eq!(AlignContent::from_str("start").unwrap(), AlignContent::Start);
        assert_eq!(AlignContent::from_str("end").unwrap(), AlignContent::End);
        assert_eq!(AlignContent::from_str("flex-start").unwrap(), AlignContent::FlexStart);
        assert_eq!(AlignContent::from_str("flex-end").unwrap(), AlignContent::FlexEnd);
        assert_eq!(AlignContent::from_str("normal").unwrap(), AlignContent::Normal);
        assert_eq!(AlignContent::from_str("space-between").unwrap(), AlignContent::SpaceBetween);
        assert_eq!(AlignContent::from_str("space-around").unwrap(), AlignContent::SpaceAround);
        assert_eq!(AlignContent::from_str("space-evenly").unwrap(), AlignContent::SpaceEvenly);
        assert_eq!(AlignContent::from_str("stretch").unwrap(), AlignContent::Stretch);
        assert_eq!(AlignContent::from_str("baseline").unwrap(), AlignContent::Baseline);
        assert!(AlignContent::from_str("invalid").is_err());
    }
}
