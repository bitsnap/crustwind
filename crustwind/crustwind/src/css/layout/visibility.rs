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
pub enum Visibility {
    #[display("visible")]
    Visible,

    #[display("invisible")]
    Invisible,

    #[display("collapse")]
    Collapse,
}

impl FromStr for Visibility {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "visible" => Ok(Visibility::Visible),
            "invisible" => Ok(Visibility::Invisible),
            "collapse" => Ok(Visibility::Collapse),
            _ => Err(anyhow::anyhow!("Invalid visibility value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_display() {
        goldie::assert_json!(
            vec![
                Visibility::Visible,
                Visibility::Invisible,
                Visibility::Collapse,
            ]
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_visibility_from_str() {
        assert_eq!(
            Visibility::from_str("visible").unwrap(),
            Visibility::Visible
        );
        assert_eq!(
            Visibility::from_str("invisible").unwrap(),
            Visibility::Invisible
        );
        assert_eq!(
            Visibility::from_str("collapse").unwrap(),
            Visibility::Collapse
        );
    }
}
