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

mod bottom;
mod left;
mod right;
mod top;

pub(crate) use bottom::*;
pub(crate) use left::*;
pub(crate) use right::*;
pub(crate) use top::*;

use derive_more::*;
use serde::*;

#[derive(derive_more::Debug, Clone, Copy, PartialEq, Display, Serialize, Deserialize)]
pub enum Position {
    #[display("static")]
    Static,

    #[display("fixed")]
    Fixed,

    #[display("absolute")]
    Absolute,

    #[display("relative")]
    Relative,

    #[display("sticky")]
    Sticky,
}
