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

use crate::css::*;

#[derive(Clone, Copy, Debug, PartialEq, Display, From)]
pub enum ColorSpace {
    #[display("srgb")]
    Srgb,

    #[display("srgb-linear")]
    SrgbLinear,

    #[display("display-p3")]
    DisplayP3,

    #[display("a98-rgb")]
    A98RGB,

    #[display("prophoto-rgb")]
    ProPhotoRGB,

    #[display("rec2020")]
    Rec2020,

    #[display("xyz")]
    XYZ,

    #[display("xyz-d50")]
    XYZD50,

    #[display("xyz-d65")]
    XYZD65,
}

#[derive(Clone, Copy, Debug, PartialEq, From)]
pub struct Color {
    space: ColorSpace,
    c1: Number,
    c2: Number,
    c3: Number,
    alpha: Number,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.alpha > Number::from(0) {
            write!(
                f,
                "color({} {} {} {} / {})",
                self.space, self.c1, self.c2, self.c3, self.alpha
            )
        } else {
            write!(
                f,
                "color({} {} {} {})",
                self.space, self.c1, self.c2, self.c3
            )
        }
    }
}
