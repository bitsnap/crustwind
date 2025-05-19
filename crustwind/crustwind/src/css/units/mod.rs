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

mod color;
mod calc;
mod number;
mod variable;

pub(crate) use color::*;
pub(crate) use calc::*;
pub(crate) use number::*;
pub(crate) use variable::*;

use crustwind_macro::css_unit;

css_unit!(Em, "em");
css_unit!(Pct, "%");
css_unit!(Px, "px");
css_unit!(Rem, "rem");
css_unit!(Vw, "vw");
css_unit!(Vh, "vh");
css_unit!(Vmin, "vmin");
css_unit!(Vmax, "vmax");

#[derive(derive_more::Debug, Clone, PartialEq, From, Serialize, Deserialize)]
pub(crate) enum Unit {
    Length(LengthUnit),
    Color(ColorUnit),
}

#[derive(derive_more::Debug, Clone, PartialEq, From, Serialize, Display, Deserialize)]
pub enum LengthUnit {
    Calc(Calc),
    Em(Em),
    Number(Number),
    Pct(Pct),
    Px(Px),
    Rem(Rem),
    Variable(Variable),
    Vw(Vw),
    Vh(Vh),
    Vmin(Vmin),
    Vmax(Vmax),
}

impl FromStr for LengthUnit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try special functions first
        if s.starts_with("calc(") && s.ends_with(")") {
            return Calc::from_str(s).map(LengthUnit::Calc);
        }

        if s.starts_with("var(") && s.ends_with(")") {
            return Variable::from_str(s).map(LengthUnit::Variable);
        }

        // Try number
        Number::from_str(s)
            .map(LengthUnit::Number)
            .or_else(|_| {
                // Try specific units
                match s {
                    _ if s.ends_with("%") => Pct::from_str(s).map(LengthUnit::Pct),
                    _ if s.ends_with("px") => Px::from_str(s).map(LengthUnit::Px),
                    _ if s.ends_with("em") => Em::from_str(s).map(LengthUnit::Em),
                    _ if s.ends_with("rem") => Rem::from_str(s).map(LengthUnit::Rem),
                    _ if s.ends_with("vw") => Vw::from_str(s).map(LengthUnit::Vw),
                    _ if s.ends_with("vh") => Vh::from_str(s).map(LengthUnit::Vh),
                    _ if s.ends_with("vmin") => Vmin::from_str(s).map(LengthUnit::Vmin),
                    _ if s.ends_with("vmax") => Vmax::from_str(s).map(LengthUnit::Vmax),
                    _ => Err(anyhow::anyhow!("Invalid length unit: {}", s)),
                }
            })
    }
}


#[derive(derive_more::Debug, Clone, PartialEq, Display, Serialize, Deserialize)]
pub enum ColorUnit {
    Calc(Calc),
    Number(Number),
    Pct(Pct),
    Variable(Variable),
}

impl FromStr for ColorUnit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try special functions first
        if s.starts_with("calc(") && s.ends_with(")") {
            return Calc::from_str(s).map(ColorUnit::Calc);
        }

        if s.starts_with("var(") && s.ends_with(")") {
            return Variable::from_str(s).map(ColorUnit::Variable);
        }

        // Try number
        Number::from_str(s)
            .map(ColorUnit::Number)
            .or_else(|_| {
                // Try percentage
                if s.ends_with("%") {
                    Pct::from_str(s).map(ColorUnit::Pct)
                } else {
                    Err(anyhow::anyhow!("Invalid color unit: {}", s))
                }
            })
    }
}
