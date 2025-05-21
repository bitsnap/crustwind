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
use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::css::color::Color as CSSColor;

/// Represents a CSS `background-color` attribute.
///
/// The `Color` type is used to represent CSS `background-color` attribute.
///
#[derive(Clone, Copy, Debug, PartialEq, Display, From, FromStr)]
#[display("{_0}")]
pub struct Color(pub CSSColor);

impl Deref for Color {
    type Target = CSSColor;

    /// Returns a reference to the inner value.
    ///
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    /// Returns a mutable reference to the inner value.
    ///
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
