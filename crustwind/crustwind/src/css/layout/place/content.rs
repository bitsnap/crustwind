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
pub enum PlaceContent {
    #[display("place-content: center")]
    Center,

    #[display("place-content: start")]
    Start,

    #[display("place-content: end")]
    End,

    #[display("place-content: flex-start")]
    FlexStart,

    #[display("place-content: flex-end")]
    FlexEnd,

    #[display("place-content: normal")]
    Normal,

    #[display("place-content: space-between")]
    SpaceBetween,

    #[display("place-content: space-around")]
    SpaceAround,

    #[display("place-content: space-evenly")]
    SpaceEvenly,

    #[display("place-content: stretch")]
    Stretch,

    #[display("place-content: baseline")]
    Baseline,
}
