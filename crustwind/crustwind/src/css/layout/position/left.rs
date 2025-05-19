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


use crate::css::units::LengthUnit;

#[derive(derive_more::Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
pub enum Left {
    #[display("left: auto")]
    Auto,
    #[display("left: {_0}")]
    Length(LengthUnit),
}
