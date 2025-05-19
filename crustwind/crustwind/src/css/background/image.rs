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

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
#[display("url({_0})")]
pub struct Image(pub String);

impl FromStr for Image {
    type Err = anyhow::Error;

    /// Parses a string into a `Variable` value.
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Image(
            s.replace("url(", "").replace(")", "").trim().to_string(),
        ))
    }
}
