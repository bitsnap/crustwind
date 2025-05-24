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
pub enum JustifyItems {
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

    #[display("self-start")]
    SelfStart,

    #[display("self-end")]
    SelfEnd,

    #[display("left")]
    Left,

    #[display("right")]
    Right,

    #[display("baseline")]
    Baseline,

    #[display("stretch")]
    Stretch,

    #[display("normal")]
    Normal,
}

impl FromStr for JustifyItems {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "center" => Ok(JustifyItems::Center),
            "start" => Ok(JustifyItems::Start),
            "end" => Ok(JustifyItems::End),
            "flex-start" => Ok(JustifyItems::FlexStart),
            "flex-end" => Ok(JustifyItems::FlexEnd),
            "self-start" => Ok(JustifyItems::SelfStart),
            "self-end" => Ok(JustifyItems::SelfEnd),
            "left" => Ok(JustifyItems::Left),
            "right" => Ok(JustifyItems::Right),
            "baseline" => Ok(JustifyItems::Baseline),
            "stretch" => Ok(JustifyItems::Stretch),
            "normal" => Ok(JustifyItems::Normal),
            _ => Err(anyhow::anyhow!("Invalid justify-items value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_justify_items_display() {
        goldie::assert_json!(
            vec![
                JustifyItems::Center,
                JustifyItems::Start,
                JustifyItems::End,
                JustifyItems::FlexStart,
                JustifyItems::FlexEnd,
                JustifyItems::SelfStart,
                JustifyItems::SelfEnd,
                JustifyItems::Left,
                JustifyItems::Right,
                JustifyItems::Baseline,
                JustifyItems::Stretch,
                JustifyItems::Normal,
            ]
            .iter()
            .map(|j| j.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_justify_items_from_str() {
        assert_eq!(JustifyItems::from_str("center").unwrap(), JustifyItems::Center);
        assert_eq!(JustifyItems::from_str("start").unwrap(), JustifyItems::Start);
        assert_eq!(JustifyItems::from_str("end").unwrap(), JustifyItems::End);
        assert_eq!(JustifyItems::from_str("flex-start").unwrap(), JustifyItems::FlexStart);
        assert_eq!(JustifyItems::from_str("flex-end").unwrap(), JustifyItems::FlexEnd);
        assert_eq!(JustifyItems::from_str("self-start").unwrap(), JustifyItems::SelfStart);
        assert_eq!(JustifyItems::from_str("self-end").unwrap(), JustifyItems::SelfEnd);
        assert_eq!(JustifyItems::from_str("left").unwrap(), JustifyItems::Left);
        assert_eq!(JustifyItems::from_str("right").unwrap(), JustifyItems::Right);
        assert_eq!(JustifyItems::from_str("baseline").unwrap(), JustifyItems::Baseline);
        assert_eq!(JustifyItems::from_str("stretch").unwrap(), JustifyItems::Stretch);
        assert_eq!(JustifyItems::from_str("normal").unwrap(), JustifyItems::Normal);
        assert!(JustifyItems::from_str("invalid").is_err());
    }
}
