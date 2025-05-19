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
pub enum ObjectPosition {
    #[display("object-position: bottom")]
    Bottom,

    #[display("object-position: center")]
    Center,

    #[display("object-position: left")]
    Left,

    #[display("object-position: left-bottom")]
    LeftBottom,

    #[display("object-position: left-top")]
    LeftTop,

    #[display("object-position: right")]
    Right,

    #[display("object-position: right-bottom")]
    RightBottom,

    #[display("object-position: right-top")]
    RightTop,

    #[display("object-position: top")]
    Top,
}
