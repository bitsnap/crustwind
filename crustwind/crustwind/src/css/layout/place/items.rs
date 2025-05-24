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
pub enum PlaceItems {
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

impl FromStr for PlaceItems {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "center" => Ok(PlaceItems::Center),
            "start" => Ok(PlaceItems::Start),
            "end" => Ok(PlaceItems::End),
            "flex-start" => Ok(PlaceItems::FlexStart),
            "flex-end" => Ok(PlaceItems::FlexEnd),
            "self-start" => Ok(PlaceItems::SelfStart),
            "self-end" => Ok(PlaceItems::SelfEnd),
            "stretch" => Ok(PlaceItems::Stretch),
            "baseline" => Ok(PlaceItems::Baseline),
            _ => Err(anyhow::anyhow!("Invalid place-items value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_items_display() {
        goldie::assert_json!(
            vec![
                PlaceItems::Center,
                PlaceItems::Start,
                PlaceItems::End,
                PlaceItems::FlexStart,
                PlaceItems::FlexEnd,
                PlaceItems::SelfStart,
                PlaceItems::SelfEnd,
                PlaceItems::Stretch,
                PlaceItems::Baseline,
            ]
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_place_items_from_str() {
        assert_eq!(PlaceItems::from_str("center").unwrap(), PlaceItems::Center);
        assert_eq!(PlaceItems::from_str("start").unwrap(), PlaceItems::Start);
        assert_eq!(PlaceItems::from_str("end").unwrap(), PlaceItems::End);
        assert_eq!(PlaceItems::from_str("flex-start").unwrap(), PlaceItems::FlexStart);
        assert_eq!(PlaceItems::from_str("flex-end").unwrap(), PlaceItems::FlexEnd);
        assert_eq!(PlaceItems::from_str("self-start").unwrap(), PlaceItems::SelfStart);
        assert_eq!(PlaceItems::from_str("self-end").unwrap(), PlaceItems::SelfEnd);
        assert_eq!(PlaceItems::from_str("stretch").unwrap(), PlaceItems::Stretch);
        assert_eq!(PlaceItems::from_str("baseline").unwrap(), PlaceItems::Baseline);
        assert!(PlaceItems::from_str("invalid").is_err());
    }
}
