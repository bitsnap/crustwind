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
