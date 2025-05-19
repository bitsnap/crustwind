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
pub enum PlaceSelf {
    #[display("place-self: auto")]
    Auto,

    #[display("place-self: center")]
    Center,

    #[display("place-self: start")]
    Start,

    #[display("place-self: end")]
    End,

    #[display("place-self: flex-start")]
    FlexStart,

    #[display("place-self: flex-end")]
    FlexEnd,

    #[display("place-self: self-start")]
    SelfStart,

    #[display("place-self: self-end")]
    SelfEnd,

    #[display("place-self: stretch")]
    Stretch,

    #[display("place-self: baseline")]
    Baseline,
}
