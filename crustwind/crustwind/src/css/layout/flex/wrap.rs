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
pub enum FlexWrap {
    #[display("nowrap")]
    NoWrap,

    #[display("wrap")]
    Wrap,

    #[display("wrap-reverse")]
    WrapReverse,
}

impl FromStr for FlexWrap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nowrap" => Ok(FlexWrap::NoWrap),
            "wrap" => Ok(FlexWrap::Wrap),
            "wrap-reverse" => Ok(FlexWrap::WrapReverse),
            _ => Err(anyhow::anyhow!("Invalid flex-wrap value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flex_wrap_display() {
        goldie::assert_json!(
            vec![
                FlexWrap::NoWrap,
                FlexWrap::Wrap,
                FlexWrap::WrapReverse,
            ]
            .iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_flex_wrap_from_str() {
        assert_eq!(FlexWrap::from_str("nowrap").unwrap(), FlexWrap::NoWrap);
        assert_eq!(FlexWrap::from_str("wrap").unwrap(), FlexWrap::Wrap);
        assert_eq!(FlexWrap::from_str("wrap-reverse").unwrap(), FlexWrap::WrapReverse);
    }
}
