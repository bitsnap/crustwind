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
pub enum PlaceSelf {
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

    #[display("stretch")]
    Stretch,

    #[display("baseline")]
    Baseline,
}

impl FromStr for PlaceSelf {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(PlaceSelf::Auto),
            "center" => Ok(PlaceSelf::Center),
            "start" => Ok(PlaceSelf::Start),
            "end" => Ok(PlaceSelf::End),
            "flex-start" => Ok(PlaceSelf::FlexStart),
            "flex-end" => Ok(PlaceSelf::FlexEnd),
            "self-start" => Ok(PlaceSelf::SelfStart),
            "self-end" => Ok(PlaceSelf::SelfEnd),
            "stretch" => Ok(PlaceSelf::Stretch),
            "baseline" => Ok(PlaceSelf::Baseline),
            _ => Err(anyhow::anyhow!("Invalid place-self value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_self_display() {
        goldie::assert_json!(
            vec![
                PlaceSelf::Auto,
                PlaceSelf::Center,
                PlaceSelf::Start,
                PlaceSelf::End,
                PlaceSelf::FlexStart,
                PlaceSelf::FlexEnd,
                PlaceSelf::SelfStart,
                PlaceSelf::SelfEnd,
                PlaceSelf::Stretch,
                PlaceSelf::Baseline,
            ]
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_place_self_from_str() {
        assert_eq!(PlaceSelf::from_str("auto").unwrap(), PlaceSelf::Auto);
        assert_eq!(PlaceSelf::from_str("center").unwrap(), PlaceSelf::Center);
        assert_eq!(PlaceSelf::from_str("start").unwrap(), PlaceSelf::Start);
        assert_eq!(PlaceSelf::from_str("end").unwrap(), PlaceSelf::End);
        assert_eq!(PlaceSelf::from_str("flex-start").unwrap(), PlaceSelf::FlexStart);
        assert_eq!(PlaceSelf::from_str("flex-end").unwrap(), PlaceSelf::FlexEnd);
        assert_eq!(PlaceSelf::from_str("self-start").unwrap(), PlaceSelf::SelfStart);
        assert_eq!(PlaceSelf::from_str("self-end").unwrap(), PlaceSelf::SelfEnd);
        assert_eq!(PlaceSelf::from_str("stretch").unwrap(), PlaceSelf::Stretch);
        assert_eq!(PlaceSelf::from_str("baseline").unwrap(), PlaceSelf::Baseline);
        assert!(PlaceSelf::from_str("invalid").is_err());
    }
}
