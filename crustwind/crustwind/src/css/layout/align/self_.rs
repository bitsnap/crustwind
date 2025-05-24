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
pub enum AlignSelf {
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

impl FromStr for AlignSelf {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(AlignSelf::Auto),
            "center" => Ok(AlignSelf::Center),
            "start" => Ok(AlignSelf::Start),
            "end" => Ok(AlignSelf::End),
            "flex-start" => Ok(AlignSelf::FlexStart),
            "flex-end" => Ok(AlignSelf::FlexEnd),
            "self-start" => Ok(AlignSelf::SelfStart),
            "self-end" => Ok(AlignSelf::SelfEnd),
            "stretch" => Ok(AlignSelf::Stretch),
            "baseline" => Ok(AlignSelf::Baseline),
            _ => Err(anyhow::anyhow!("Invalid align-self value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_self_display() {
        goldie::assert_json!(
            vec![
                AlignSelf::Auto,
                AlignSelf::Center,
                AlignSelf::Start,
                AlignSelf::End,
                AlignSelf::FlexStart,
                AlignSelf::FlexEnd,
                AlignSelf::SelfStart,
                AlignSelf::SelfEnd,
                AlignSelf::Stretch,
                AlignSelf::Baseline,
            ]
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_align_self_from_str() {
        assert_eq!(AlignSelf::from_str("auto").unwrap(), AlignSelf::Auto);
        assert_eq!(AlignSelf::from_str("center").unwrap(), AlignSelf::Center);
        assert_eq!(AlignSelf::from_str("start").unwrap(), AlignSelf::Start);
        assert_eq!(AlignSelf::from_str("end").unwrap(), AlignSelf::End);
        assert_eq!(AlignSelf::from_str("flex-start").unwrap(), AlignSelf::FlexStart);
        assert_eq!(AlignSelf::from_str("flex-end").unwrap(), AlignSelf::FlexEnd);
        assert_eq!(AlignSelf::from_str("self-start").unwrap(), AlignSelf::SelfStart);
        assert_eq!(AlignSelf::from_str("self-end").unwrap(), AlignSelf::SelfEnd);
        assert_eq!(AlignSelf::from_str("stretch").unwrap(), AlignSelf::Stretch);
        assert_eq!(AlignSelf::from_str("baseline").unwrap(), AlignSelf::Baseline);
        assert!(AlignSelf::from_str("invalid").is_err());
    }
}
