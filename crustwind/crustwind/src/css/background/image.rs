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

pub type Image = String;

impl From<String> for Image {
    fn from(s: String) -> Self {
        format!("url({})", s)
    }
}

impl From<&str> for Image {
    fn from(s: &str) -> Self {
        format!("url({})", s)
    }
}
