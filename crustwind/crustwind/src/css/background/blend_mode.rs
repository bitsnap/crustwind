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

#[derive(derive_more::Debug, Clone, Copy, PartialEq, Display, From, FromStr)]
pub enum BlendMode {
    #[display("normal")]
    Normal,
    #[display("multiply")]
    Multiply,
    #[display("hard-light")]
    HardLight,
    #[display("difference")]
    Difference,
}
