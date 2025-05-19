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
pub enum Padding {
    #[display("padding: {_0}")]
    Padding(LengthUnit),
    #[display("padding-inline: {_0}")]
    PaddingInline(LengthUnit),
    #[display("padding-block: {_0}")]
    PaddingBlock(LengthUnit),
    #[display("padding-inline-start: {_0}")]
    PaddingInlineStart(LengthUnit),
    #[display("padding-inline-end: {_0}")]
    PaddingInlineEnd(LengthUnit),
    #[display("padding-top: {_0}")]
    PaddingTop(LengthUnit),
    #[display("padding-right: {_0}")]
    PaddingRight(LengthUnit),
    #[display("padding-bottom: {_0}")]
    PaddingBottom(LengthUnit),
    #[display("padding-left: {_0}")]
    PaddingLeft(LengthUnit),
}
