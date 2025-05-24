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
pub enum ObjectPosition {
    #[display("bottom")]
    Bottom,

    #[display("center")]
    Center,

    #[display("left")]
    Left,

    #[display("left-bottom")]
    LeftBottom,

    #[display("left-top")]
    LeftTop,

    #[display("right")]
    Right,

    #[display("right-bottom")]
    RightBottom,

    #[display("right-top")]
    RightTop,

    #[display("top")]
    Top,
}

impl FromStr for ObjectPosition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bottom" => Ok(ObjectPosition::Bottom),
            "center" => Ok(ObjectPosition::Center),
            "left" => Ok(ObjectPosition::Left),
            "left-bottom" => Ok(ObjectPosition::LeftBottom),
            "left-top" => Ok(ObjectPosition::LeftTop),
            "right" => Ok(ObjectPosition::Right),
            "right-bottom" => Ok(ObjectPosition::RightBottom),
            "right-top" => Ok(ObjectPosition::RightTop),
            "top" => Ok(ObjectPosition::Top),
            _ => Err(anyhow::anyhow!("Invalid object-position value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_position_display() {
        goldie::assert_json!(
            vec![
                ObjectPosition::Bottom,
                ObjectPosition::Center,
                ObjectPosition::Left,
                ObjectPosition::LeftBottom,
                ObjectPosition::LeftTop,
                ObjectPosition::Right,
                ObjectPosition::RightBottom,
                ObjectPosition::RightTop,
                ObjectPosition::Top,
            ]
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_object_position_from_str() {
        assert_eq!(
            ObjectPosition::from_str("bottom").unwrap(),
            ObjectPosition::Bottom
        );
        assert_eq!(
            ObjectPosition::from_str("center").unwrap(),
            ObjectPosition::Center
        );
        assert_eq!(
            ObjectPosition::from_str("left").unwrap(),
            ObjectPosition::Left
        );
        assert_eq!(
            ObjectPosition::from_str("left-bottom").unwrap(),
            ObjectPosition::LeftBottom
        );
        assert_eq!(
            ObjectPosition::from_str("left-top").unwrap(),
            ObjectPosition::LeftTop
        );
        assert_eq!(
            ObjectPosition::from_str("right").unwrap(),
            ObjectPosition::Right
        );
        assert_eq!(
            ObjectPosition::from_str("right-bottom").unwrap(),
            ObjectPosition::RightBottom
        );
        assert_eq!(
            ObjectPosition::from_str("right-top").unwrap(),
            ObjectPosition::RightTop
        );
        assert_eq!(
            ObjectPosition::from_str("top").unwrap(),
            ObjectPosition::Top
        );
    }
}
