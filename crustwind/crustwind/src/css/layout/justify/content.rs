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
pub enum JustifyContent {
    #[display("justify-content: center")]
    Center,

    #[display("justify-content: start")]
    Start,

    #[display("justify-content: end")]
    End,

    #[display("justify-content: flex-start")]
    FlexStart,

    #[display("justify-content: flex-end")]
    FlexEnd,

    #[display("justify-content: left")]
    Left,

    #[display("justify-content: right")]
    Right,

    #[display("justify-content: normal")]
    Normal,

    #[display("justify-content: space-between")]
    SpaceBetween,

    #[display("justify-content: space-around")]
    SpaceAround,

    #[display("justify-content: space-evenly")]
    SpaceEvenly,

    #[display("justify-content: stretch")]
    Stretch,
}
