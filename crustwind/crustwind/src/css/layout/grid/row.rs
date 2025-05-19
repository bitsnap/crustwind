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

#[derive(derive_more::Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[display("row")]
pub struct GridRow {
    pub start: GridRowValue,
    pub end: Option<GridRowValue>,
}

#[derive(derive_more::Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
#[display("row-value")]
pub enum GridRowValue {
    #[display("grid-row: auto")]
    Auto,

    #[display("grid-row: {_0}")]
    Number(i32),

    #[display("grid-row: span {_0}")]
    Span(i32),
}
