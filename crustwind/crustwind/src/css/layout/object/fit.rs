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
pub enum ObjectFit {
    #[display("contain")]
    Contain,

    #[display("cover")]
    Cover,

    #[display("fill")]
    Fill,

    #[display("none")]
    None,

    #[display("scale-down")]
    ScaleDown,
}

impl FromStr for ObjectFit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "contain" => Ok(ObjectFit::Contain),
            "cover" => Ok(ObjectFit::Cover),
            "fill" => Ok(ObjectFit::Fill),
            "none" => Ok(ObjectFit::None),
            "scale-down" => Ok(ObjectFit::ScaleDown),
            _ => Err(anyhow::anyhow!("Invalid object-fit value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_fit_display() {
        goldie::assert_json!(
            vec![
                ObjectFit::Contain,
                ObjectFit::Cover,
                ObjectFit::Fill,
                ObjectFit::None,
                ObjectFit::ScaleDown,
            ]
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_object_fit_from_str() {
        assert_eq!(ObjectFit::from_str("contain").unwrap(), ObjectFit::Contain);
        assert_eq!(ObjectFit::from_str("cover").unwrap(), ObjectFit::Cover);
        assert_eq!(ObjectFit::from_str("fill").unwrap(), ObjectFit::Fill);
        assert_eq!(ObjectFit::from_str("none").unwrap(), ObjectFit::None);
        assert_eq!(
            ObjectFit::from_str("scale-down").unwrap(),
            ObjectFit::ScaleDown
        );
    }
}
