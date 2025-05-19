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


use crate::css::units::LengthUnit;

#[derive(derive_more::Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
pub enum Margin {
    #[display("margin: {_0}")]
    Margin(LengthUnit),
    #[display("margin-inline: {_0}")]
    MarginInline(LengthUnit),
    #[display("margin-block: {_0}")]
    MarginBlock(LengthUnit),
    #[display("margin-inline-start: {_0}")]
    MarginInlineStart(LengthUnit),
    #[display("margin-inline-end: {_0}")]
    MarginInlineEnd(LengthUnit),
    #[display("margin-top: {_0}")]
    MarginTop(LengthUnit),
    #[display("margin-right: {_0}")]
    MarginRight(LengthUnit),
    #[display("margin-bottom: {_0}")]
    MarginBottom(LengthUnit),
    #[display("margin-left: {_0}")]
    MarginLeft(LengthUnit),
}
