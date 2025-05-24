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

mod aspect_ratio;
mod box_;
mod break_;
mod clear;
mod columns;
mod display;
mod float;
mod object;
mod overflow;
mod overscroll;
mod position;
mod visibility;
mod z_index;

pub(crate) use aspect_ratio::*;
pub(crate) use box_::*;
pub(crate) use break_::*;
pub(crate) use clear::*;
pub(crate) use columns::*;
pub(crate) use display::*;
pub(crate) use float::*;
pub(crate) use object::*;
pub(crate) use overflow::*;
pub(crate) use overscroll::*;
pub(crate) use position::*;
pub(crate) use visibility::*;
pub(crate) use z_index::*;

pub(crate) fn display_opt<T: ToString>(v: &Option<T>) -> String {
    v.as_ref()
        .map_or("".into(), |s| format!(" {}", s.to_string()))
}
