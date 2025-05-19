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
pub enum JustifyItems {
    #[display("justify-items: center")]
    Center,

    #[display("justify-items: start")]
    Start,

    #[display("justify-items: end")]
    End,

    #[display("justify-items: flex-start")]
    FlexStart,

    #[display("justify-items: flex-end")]
    FlexEnd,

    #[display("justify-items: self-start")]
    SelfStart,

    #[display("justify-items: self-end")]
    SelfEnd,

    #[display("justify-items: left")]
    Left,

    #[display("justify-items: right")]
    Right,

    #[display("justify-items: baseline")]
    Baseline,

    #[display("justify-items: stretch")]
    Stretch,

    #[display("justify-items: normal")]
    Normal,
}
