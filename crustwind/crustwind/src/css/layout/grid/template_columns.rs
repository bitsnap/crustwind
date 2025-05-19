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

use crate::css::units::LengthUnit;
use derive_more::*;
use serde::*;


#[derive(derive_more::Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
pub enum GridTemplateColumns {
    #[display("none")]
    None,

    #[display("{_0}")]
    Template(GridTemplate),
}

#[derive(derive_more::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridTemplate(pub Vec<GridTemplateValue>);

impl std::fmt::Display for GridTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values: Vec<String> = self.0.iter().map(|v| v.to_string()).collect();
        write!(f, "{}", values.join(" "))
    }
}

#[derive(derive_more::Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
pub enum GridTemplateValue {
    #[display("auto")]
    Auto,

    #[display("min-content")]
    MinContent,

    #[display("max-content")]
    MaxContent,

    #[display("{_0}fr")]
    Fr(f32),

    #[display("{_0}")]
    Length(LengthUnit),
}
