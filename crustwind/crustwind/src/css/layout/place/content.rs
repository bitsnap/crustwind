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
pub enum PlaceContent {
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

impl FromStr for PlaceContent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "center" => Ok(PlaceContent::Center),
            "start" => Ok(PlaceContent::Start),
            "end" => Ok(PlaceContent::End),
            "flex-start" => Ok(PlaceContent::FlexStart),
            "flex-end" => Ok(PlaceContent::FlexEnd),
            "normal" => Ok(PlaceContent::Normal),
            "space-between" => Ok(PlaceContent::SpaceBetween),
            "space-around" => Ok(PlaceContent::SpaceAround),
            "space-evenly" => Ok(PlaceContent::SpaceEvenly),
            "stretch" => Ok(PlaceContent::Stretch),
            "baseline" => Ok(PlaceContent::Baseline),
            _ => Err(anyhow::anyhow!("Invalid place-content value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_content_display() {
        goldie::assert_json!(
            vec![
                PlaceContent::Center,
                PlaceContent::Start,
                PlaceContent::End,
                PlaceContent::FlexStart,
                PlaceContent::FlexEnd,
                PlaceContent::Normal,
                PlaceContent::SpaceBetween,
                PlaceContent::SpaceAround,
                PlaceContent::SpaceEvenly,
                PlaceContent::Stretch,
                PlaceContent::Baseline,
            ]
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_place_content_from_str() {
        assert_eq!(PlaceContent::from_str("center").unwrap(), PlaceContent::Center);
        assert_eq!(PlaceContent::from_str("start").unwrap(), PlaceContent::Start);
        assert_eq!(PlaceContent::from_str("end").unwrap(), PlaceContent::End);
        assert_eq!(PlaceContent::from_str("flex-start").unwrap(), PlaceContent::FlexStart);
        assert_eq!(PlaceContent::from_str("flex-end").unwrap(), PlaceContent::FlexEnd);
        assert_eq!(PlaceContent::from_str("normal").unwrap(), PlaceContent::Normal);
        assert_eq!(PlaceContent::from_str("space-between").unwrap(), PlaceContent::SpaceBetween);
        assert_eq!(PlaceContent::from_str("space-around").unwrap(), PlaceContent::SpaceAround);
        assert_eq!(PlaceContent::from_str("space-evenly").unwrap(), PlaceContent::SpaceEvenly);
        assert_eq!(PlaceContent::from_str("stretch").unwrap(), PlaceContent::Stretch);
        assert_eq!(PlaceContent::from_str("baseline").unwrap(), PlaceContent::Baseline);
        assert!(PlaceContent::from_str("invalid").is_err());
    }
}
