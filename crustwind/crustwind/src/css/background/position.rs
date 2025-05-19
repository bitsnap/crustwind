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
use serde::{Serialize, Deserialize};

#[derive(derive_more::Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
pub enum BackgroundPosition {
    #[display("background-position: left top")]
    LeftTop,
    #[display("background-position: right top")]
    RightTop,
    #[display("background-position: left bottom")]
    LeftBottom,
    #[display("background-position: right bottom")]
    RightBottom,
    #[display("background-position: center")]
    Center,
}
