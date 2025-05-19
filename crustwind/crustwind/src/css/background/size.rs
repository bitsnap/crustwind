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

#[derive(derive_more::Debug, Clone, PartialEq, Display, From)]
pub enum Size {
    // #[display(fmt = "{} {}", _0, _1)]
    // WidthHeight(LengthPercent, LengthPercent),
    #[display("auto")]
    Auto,
    #[display("cover")]
    Cover,
    #[display("contain")]
    Contain,
}
