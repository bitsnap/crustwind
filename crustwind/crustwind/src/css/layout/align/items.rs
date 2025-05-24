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
pub enum AlignItems {
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

impl FromStr for AlignItems {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "center" => Ok(AlignItems::Center),
            "start" => Ok(AlignItems::Start),
            "end" => Ok(AlignItems::End),
            "flex-start" => Ok(AlignItems::FlexStart),
            "flex-end" => Ok(AlignItems::FlexEnd),
            "self-start" => Ok(AlignItems::SelfStart),
            "self-end" => Ok(AlignItems::SelfEnd),
            "stretch" => Ok(AlignItems::Stretch),
            "baseline" => Ok(AlignItems::Baseline),
            _ => Err(anyhow::anyhow!("Invalid align-items value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_items_display() {
        goldie::assert_json!(
            vec![
                AlignItems::Center,
                AlignItems::Start,
                AlignItems::End,
                AlignItems::FlexStart,
                AlignItems::FlexEnd,
                AlignItems::SelfStart,
                AlignItems::SelfEnd,
                AlignItems::Stretch,
                AlignItems::Baseline,
            ]
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_align_items_from_str() {
        assert_eq!(AlignItems::from_str("center").unwrap(), AlignItems::Center);
        assert_eq!(AlignItems::from_str("start").unwrap(), AlignItems::Start);
        assert_eq!(AlignItems::from_str("end").unwrap(), AlignItems::End);
        assert_eq!(AlignItems::from_str("flex-start").unwrap(), AlignItems::FlexStart);
        assert_eq!(AlignItems::from_str("flex-end").unwrap(), AlignItems::FlexEnd);
        assert_eq!(AlignItems::from_str("self-start").unwrap(), AlignItems::SelfStart);
        assert_eq!(AlignItems::from_str("self-end").unwrap(), AlignItems::SelfEnd);
        assert_eq!(AlignItems::from_str("stretch").unwrap(), AlignItems::Stretch);
        assert_eq!(AlignItems::from_str("baseline").unwrap(), AlignItems::Baseline);
        assert!(AlignItems::from_str("invalid").is_err());
    }
}
