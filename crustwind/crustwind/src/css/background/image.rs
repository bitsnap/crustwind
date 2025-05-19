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

#[derive(derive_more::Debug, Clone, PartialEq, Display, From, Serialize, Deserialize)]
#[display("background-image: url({_0})")]
pub struct BackgroundImage(pub String);

impl std::str::FromStr for BackgroundImage {
    type Err = anyhow::Error;

    /// Parses a string into a `Variable` value.
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BackgroundImage(
            s.replace("url(", "").replace(")", "").trim().to_string(),
        ))
    }
}
