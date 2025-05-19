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
pub enum PlaceItems {
    #[display("place-items: center")]
    Center,

    #[display("place-items: start")]
    Start,

    #[display("place-items: end")]
    End,

    #[display("place-items: flex-start")]
    FlexStart,

    #[display("place-items: flex-end")]
    FlexEnd,

    #[display("place-items: self-start")]
    SelfStart,

    #[display("place-items: self-end")]
    SelfEnd,

    #[display("place-items: stretch")]
    Stretch,

    #[display("place-items: baseline")]
    Baseline,
}
