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

use serde::*;

#[derive(derive_more::Debug, Clone, PartialEq, Display, From, Serialize, Deserialize)]
pub struct Calc(#[display("calc({})")] pub String);

impl Deref for Calc {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Calc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Calc {
    type Err = anyhow::Error;

    /// Parses a string into a `Calc` value.
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Calc(
            s.replace("calc(", "").replace(")", "").trim().to_string(),
        ))
    }
}
