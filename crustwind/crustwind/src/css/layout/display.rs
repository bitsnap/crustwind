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


#[derive(derive_more::Debug, Clone, Copy, PartialEq, Display, Serialize, Deserialize)]
pub enum Display {
    #[display("display: block")]
    Block,

    #[display("display: inline-block")]
    InlineBlock,

    #[display("display: inline")]
    Inline,

    #[display("display: flex")]
    Flex,

    #[display("display: inline-flex")]
    InlineFlex,

    #[display("display: table")]
    Table,

    #[display("display: inline-table")]
    InlineTable,

    #[display("display: table-caption")]
    TableCaption,

    #[display("display: table-cell")]
    TableCell,

    #[display("display: table-column")]
    TableColumn,

    #[display("display: table-column-group")]
    TableColumnGroup,

    #[display("display: table-footer-group")]
    TableFooterGroup,

    #[display("display: table-header-group")]
    TableHeaderGroup,

    #[display("display: table-row-group")]
    TableRowGroup,

    #[display("display: table-row")]
    TableRow,

    #[display("display: flow-root")]
    FlowRoot,

    #[display("display: grid")]
    Grid,

    #[display("display: inline-grid")]
    InlineGrid,

    #[display("display: contents")]
    Contents,

    #[display("display: list-item")]
    ListItem,

    #[display("display: none")]
    None,
}
