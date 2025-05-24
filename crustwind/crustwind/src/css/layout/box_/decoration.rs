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
pub enum BoxDecoration {
    #[display("slice")]
    Slice,

    #[display("clone")]
    Clone,
}

impl FromStr for BoxDecoration {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "slice" => Ok(BoxDecoration::Slice),
            "clone" => Ok(BoxDecoration::Clone),
            _ => Err(anyhow::anyhow!("Invalid box-decoration value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_decoration_display() {
        goldie::assert_json!(
            vec![BoxDecoration::Slice, BoxDecoration::Clone,]
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_box_decoration_from_str() {
        assert_eq!(
            BoxDecoration::from_str("slice").unwrap(),
            BoxDecoration::Slice
        );
        assert_eq!(
            BoxDecoration::from_str("clone").unwrap(),
            BoxDecoration::Clone
        );
    }
}
