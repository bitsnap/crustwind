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

use rust_decimal::*;

#[derive(derive_more::Debug, Clone, Copy, PartialEq, PartialOrd, Display, Serialize, Deserialize)]
#[display("{_0}")]
pub struct Number(pub Decimal);

impl Deref for Number {
    type Target = Decimal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Number {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number(value.into())
    }
}

impl FromStr for Number {
    type Err = rust_decimal::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        s.parse().map(|n| Self(n))
    }
}
