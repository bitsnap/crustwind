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

use crate::css::units::LengthUnit;

#[derive(derive_more::Debug, Clone, PartialEq, Display, From, Serialize, Deserialize)]
pub enum BackgroundSize {
    #[display("background-size: {_0} {_1}")]
    WidthHeight(LengthUnit, LengthUnit),

    #[display("background-size: auto")]
    Auto,
    #[display("background-size: cover")]
    Cover,
    #[display("background-size: contain")]
    Contain,
}
