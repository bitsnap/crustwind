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
mod attachment;
mod blend_mode;
mod clip;
mod color;
mod origin;
mod position;
mod repeat;
mod size;

pub use attachment::*;
pub use blend_mode::*;
pub use clip::*;
pub use color::*;
pub use origin::*;
pub use position::*;
pub use repeat::*;
pub use size::*;

pub(crate) fn display_opt<T: ToString>(v: &Option<T>) -> String {
    v.as_ref()
        .map_or("".into(), |s| format!(" {}", s.to_string()))
}

pub enum Background {}
