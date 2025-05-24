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
pub enum BoxSizing {
    #[display("border-box")]
    BorderBox,

    #[display("content-box")]
    ContentBox,
}

impl FromStr for BoxSizing {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "border-box" => Ok(BoxSizing::BorderBox),
            "content-box" => Ok(BoxSizing::ContentBox),
            _ => Err(anyhow::anyhow!("Invalid box-sizing value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_sizing_display() {
        goldie::assert_json!(
            vec![BoxSizing::BorderBox, BoxSizing::ContentBox,]
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_box_sizing_from_str() {
        assert_eq!(
            BoxSizing::from_str("border-box").unwrap(),
            BoxSizing::BorderBox
        );
        assert_eq!(
            BoxSizing::from_str("content-box").unwrap(),
            BoxSizing::ContentBox
        );
    }
}
