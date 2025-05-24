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
pub struct Order(pub i32);

impl FromStr for Order {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<i32>()
            .map(Order)
            .map_err(|_| anyhow::anyhow!("Invalid order value: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_display() {
        goldie::assert_json!(
            vec![Order(-1), Order(0), Order(1), Order(10)]
                .iter()
                .map(|o| o.to_string())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_order_from_str() {
        assert_eq!(Order::from_str("-1").unwrap(), Order(-1));
        assert_eq!(Order::from_str("0").unwrap(), Order(0));
        assert_eq!(Order::from_str("1").unwrap(), Order(1));
        assert_eq!(Order::from_str("10").unwrap(), Order(10));
        assert!(Order::from_str("invalid").is_err());
    }
}
