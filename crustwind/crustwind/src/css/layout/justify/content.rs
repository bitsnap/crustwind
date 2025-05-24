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
pub enum JustifyContent {
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

    #[display("left")]
    Left,

    #[display("right")]
    Right,

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
}

impl FromStr for JustifyContent {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "center" => Ok(JustifyContent::Center),
            "start" => Ok(JustifyContent::Start),
            "end" => Ok(JustifyContent::End),
            "flex-start" => Ok(JustifyContent::FlexStart),
            "flex-end" => Ok(JustifyContent::FlexEnd),
            "left" => Ok(JustifyContent::Left),
            "right" => Ok(JustifyContent::Right),
            "normal" => Ok(JustifyContent::Normal),
            "space-between" => Ok(JustifyContent::SpaceBetween),
            "space-around" => Ok(JustifyContent::SpaceAround),
            "space-evenly" => Ok(JustifyContent::SpaceEvenly),
            "stretch" => Ok(JustifyContent::Stretch),
            _ => Err(anyhow::anyhow!("Invalid justify-content value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_justify_content_display() {
        goldie::assert_json!(
            vec![
                JustifyContent::Center,
                JustifyContent::Start,
                JustifyContent::End,
                JustifyContent::FlexStart,
                JustifyContent::FlexEnd,
                JustifyContent::Left,
                JustifyContent::Right,
                JustifyContent::Normal,
                JustifyContent::SpaceBetween,
                JustifyContent::SpaceAround,
                JustifyContent::SpaceEvenly,
                JustifyContent::Stretch,
            ]
            .iter()
            .map(|j| j.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_justify_content_from_str() {
        assert_eq!(JustifyContent::from_str("center").unwrap(), JustifyContent::Center);
        assert_eq!(JustifyContent::from_str("start").unwrap(), JustifyContent::Start);
        assert_eq!(JustifyContent::from_str("end").unwrap(), JustifyContent::End);
        assert_eq!(JustifyContent::from_str("flex-start").unwrap(), JustifyContent::FlexStart);
        assert_eq!(JustifyContent::from_str("flex-end").unwrap(), JustifyContent::FlexEnd);
        assert_eq!(JustifyContent::from_str("left").unwrap(), JustifyContent::Left);
        assert_eq!(JustifyContent::from_str("right").unwrap(), JustifyContent::Right);
        assert_eq!(JustifyContent::from_str("normal").unwrap(), JustifyContent::Normal);
        assert_eq!(JustifyContent::from_str("space-between").unwrap(), JustifyContent::SpaceBetween);
        assert_eq!(JustifyContent::from_str("space-around").unwrap(), JustifyContent::SpaceAround);
        assert_eq!(JustifyContent::from_str("space-evenly").unwrap(), JustifyContent::SpaceEvenly);
        assert_eq!(JustifyContent::from_str("stretch").unwrap(), JustifyContent::Stretch);
        assert!(JustifyContent::from_str("invalid").is_err());
    }
}
