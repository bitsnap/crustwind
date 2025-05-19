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

use derive_more::*;
use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

/// Represents a CSS `variable` unit.
///
/// The `Variable` type is used to represent CSS variables, which are used to store and reuse values in CSS.
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Display)]
pub struct Variable(#[display("var(--{})")] pub String);

impl Deref for Variable {
    type Target = String;

    /// Returns a reference to the inner value.
    ///
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Variable {
    /// Returns a mutable reference to the inner value.
    ///
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Variable {
    type Err = anyhow::Error;

    /// Parses a string into a `Variable` value.
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Variable(
            s.replace("var(--", "").replace(")", "").trim().to_string(),
        ))
    }
}
