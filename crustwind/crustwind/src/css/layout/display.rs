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

#[derive(derive_more::Debug, Clone, Copy, PartialEq, Display, Serialize, Deserialize)]
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
