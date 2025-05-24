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
pub enum Display {
    #[display("block")]
    Block,

    #[display("inline-block")]
    InlineBlock,

    #[display("inline")]
    Inline,

    #[display("flex")]
    Flex,

    #[display("inline-flex")]
    InlineFlex,

    #[display("table")]
    Table,

    #[display("inline-table")]
    InlineTable,

    #[display("table-caption")]
    TableCaption,

    #[display("table-cell")]
    TableCell,

    #[display("table-column")]
    TableColumn,

    #[display("table-column-group")]
    TableColumnGroup,

    #[display("table-footer-group")]
    TableFooterGroup,

    #[display("table-header-group")]
    TableHeaderGroup,

    #[display("table-row-group")]
    TableRowGroup,

    #[display("table-row")]
    TableRow,

    #[display("flow-root")]
    FlowRoot,

    #[display("grid")]
    Grid,

    #[display("inline-grid")]
    InlineGrid,

    #[display("contents")]
    Contents,

    #[display("list-item")]
    ListItem,

    #[display("none")]
    None,
}

impl FromStr for Display {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "block" => Ok(Display::Block),
            "inline-block" => Ok(Display::InlineBlock),
            "inline" => Ok(Display::Inline),
            "flex" => Ok(Display::Flex),
            "inline-flex" => Ok(Display::InlineFlex),
            "table" => Ok(Display::Table),
            "inline-table" => Ok(Display::InlineTable),
            "table-caption" => Ok(Display::TableCaption),
            "table-cell" => Ok(Display::TableCell),
            "table-column" => Ok(Display::TableColumn),
            "table-column-group" => Ok(Display::TableColumnGroup),
            "table-footer-group" => Ok(Display::TableFooterGroup),
            "table-header-group" => Ok(Display::TableHeaderGroup),
            "table-row-group" => Ok(Display::TableRowGroup),
            "table-row" => Ok(Display::TableRow),
            "flow-root" => Ok(Display::FlowRoot),
            "grid" => Ok(Display::Grid),
            "inline-grid" => Ok(Display::InlineGrid),
            "contents" => Ok(Display::Contents),
            "list-item" => Ok(Display::ListItem),
            "none" => Ok(Display::None),
            _ => Err(anyhow::anyhow!("Invalid display value: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_display() {
        goldie::assert_json!(
            vec![
                Display::Block,
                Display::InlineBlock,
                Display::Inline,
                Display::Flex,
                Display::InlineFlex,
                Display::Grid,
                Display::InlineGrid,
                Display::FlowRoot,
                Display::Contents,
                Display::None,
            ]
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_display_from_str() {
        assert_eq!(Display::from_str("block").unwrap(), Display::Block);
        assert_eq!(
            Display::from_str("inline-block").unwrap(),
            Display::InlineBlock
        );
        assert_eq!(Display::from_str("inline").unwrap(), Display::Inline);
        assert_eq!(Display::from_str("flex").unwrap(), Display::Flex);
        assert_eq!(
            Display::from_str("inline-flex").unwrap(),
            Display::InlineFlex
        );
        assert_eq!(Display::from_str("grid").unwrap(), Display::Grid);
        assert_eq!(
            Display::from_str("inline-grid").unwrap(),
            Display::InlineGrid
        );
        assert_eq!(Display::from_str("flow-root").unwrap(), Display::FlowRoot);
        assert_eq!(Display::from_str("contents").unwrap(), Display::Contents);
        assert_eq!(Display::from_str("none").unwrap(), Display::None);
    }
}
