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
pub enum AlignItems {
    #[display("align-items: center")]
    Center,

    #[display("align-items: start")]
    Start,

    #[display("align-items: end")]
    End,

    #[display("align-items: flex-start")]
    FlexStart,

    #[display("align-items: flex-end")]
    FlexEnd,

    #[display("align-items: self-start")]
    SelfStart,

    #[display("align-items: self-end")]
    SelfEnd,

    #[display("align-items: stretch")]
    Stretch,

    #[display("align-items: baseline")]
    Baseline,
}
