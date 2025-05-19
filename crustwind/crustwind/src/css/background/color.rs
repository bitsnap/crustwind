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
use std::ops::{Deref, DerefMut};

use crate::css::units::Color as CSSColor;

/// Represents a CSS `background-color` attribute.
///
/// The `Color` type is used to represent CSS `background-color` attribute.
///
#[derive(derive_more::Debug, Clone, Copy, PartialEq, Display, From, FromStr, Serialize, Deserialize)]
#[display("background-color: {_0}")]
pub struct BackgroundColor(pub CSSColor);

impl Deref for BackgroundColor {
    type Target = CSSColor;

    /// Returns a reference to the inner value.
    ///
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BackgroundColor {
    /// Returns a mutable reference to the inner value.
    ///
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
