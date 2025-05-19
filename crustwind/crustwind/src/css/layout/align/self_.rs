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
pub enum AlignSelf {
    #[display("align-self: auto")]
    Auto,

    #[display("align-self: center")]
    Center,

    #[display("align-self: start")]
    Start,

    #[display("align-self: end")]
    End,

    #[display("align-self: flex-start")]
    FlexStart,

    #[display("align-self: flex-end")]
    FlexEnd,

    #[display("align-self: self-start")]
    SelfStart,

    #[display("align-self: self-end")]
    SelfEnd,

    #[display("align-self: stretch")]
    Stretch,

    #[display("align-self: baseline")]
    Baseline,
}
