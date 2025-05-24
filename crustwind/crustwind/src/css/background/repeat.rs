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

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum Repeat {
    #[display("repeat-x")]
    RepeatX,
    #[display("repeat-y")]
    RepeatY,
    #[display("repeat")]
    Repeat,
    #[display("space")]
    Space,
    #[display("round")]
    Round,
    #[display("no-repeat")]
    NoRepeat,
    #[display("initial")]
    Initial,
}
