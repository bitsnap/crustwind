/**
 * Copyright (C) 2016-2025 Yuriy Yarosh
 * All rights reserved.
 *
 * SPDX-License-Identifier: MPL-2.0
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub(crate) mod background;
pub(crate) use background::*;

pub(crate) mod color;
pub(crate) use color::*;

pub(crate) mod units;
pub(crate) use units::*;

pub(crate) mod rule;
pub use rule::*;

enum Attribute {
    Keyword(String),
    Function(String),
}
