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
pub enum JustifySelf {
    #[display("auto")]
    Auto,

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

impl FromStr for JustifySelf {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(JustifySelf::Auto),
            "center" => Ok(JustifySelf::Center),
            "start" => Ok(JustifySelf::Start),
            "end" => Ok(JustifySelf::End),
            "flex-start" => Ok(JustifySelf::FlexStart),
            "flex-end" => Ok(JustifySelf::FlexEnd),
            "self-start" => Ok(JustifySelf::SelfStart),
            "self-end" => Ok(JustifySelf::SelfEnd),
            "left" => Ok(JustifySelf::Left),
            "right" => Ok(JustifySelf::Right),
            "baseline" => Ok(JustifySelf::Baseline),
            "stretch" => Ok(JustifySelf::Stretch),
            "normal" => Ok(JustifySelf::Normal),
            _ => Err(anyhow::anyhow!("Invalid justify-self value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_justify_self_display() {
        goldie::assert_json!(
            vec![
                JustifySelf::Auto,
                JustifySelf::Center,
                JustifySelf::Start,
                JustifySelf::End,
                JustifySelf::FlexStart,
                JustifySelf::FlexEnd,
                JustifySelf::SelfStart,
                JustifySelf::SelfEnd,
                JustifySelf::Left,
                JustifySelf::Right,
                JustifySelf::Baseline,
                JustifySelf::Stretch,
                JustifySelf::Normal,
            ]
            .iter()
            .map(|j| j.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_justify_self_from_str() {
        assert_eq!(JustifySelf::from_str("auto").unwrap(), JustifySelf::Auto);
        assert_eq!(JustifySelf::from_str("center").unwrap(), JustifySelf::Center);
        assert_eq!(JustifySelf::from_str("start").unwrap(), JustifySelf::Start);
        assert_eq!(JustifySelf::from_str("end").unwrap(), JustifySelf::End);
        assert_eq!(JustifySelf::from_str("flex-start").unwrap(), JustifySelf::FlexStart);
        assert_eq!(JustifySelf::from_str("flex-end").unwrap(), JustifySelf::FlexEnd);
        assert_eq!(JustifySelf::from_str("self-start").unwrap(), JustifySelf::SelfStart);
        assert_eq!(JustifySelf::from_str("self-end").unwrap(), JustifySelf::SelfEnd);
        assert_eq!(JustifySelf::from_str("left").unwrap(), JustifySelf::Left);
        assert_eq!(JustifySelf::from_str("right").unwrap(), JustifySelf::Right);
        assert_eq!(JustifySelf::from_str("baseline").unwrap(), JustifySelf::Baseline);
        assert_eq!(JustifySelf::from_str("stretch").unwrap(), JustifySelf::Stretch);
        assert_eq!(JustifySelf::from_str("normal").unwrap(), JustifySelf::Normal);
        assert!(JustifySelf::from_str("invalid").is_err());
    }
}
