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

mod calc;
mod number;
mod variable;

use calc::*;
pub(crate) use number::*;
use variable::*;

use crustwind_macro::css_unit;

css_unit!(Em, "em");
css_unit!(Pct, "%");
css_unit!(Px, "px");
css_unit!(Rem, "rem");
css_unit!(Vw, "vw");
css_unit!(Vh, "vh");
css_unit!(Vmin, "vmin");
css_unit!(Vmax, "vmax");

pub(crate) enum Unit {
    Length(LengthUnit),
    Color(ColorUnit),
}

pub(crate) enum LengthUnit {
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

pub(crate) enum ColorUnit {
    Calc(Calc),
    Number(Number),
    Pct(Pct),
    Variable(Variable),
}
