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

mod background;
mod sizing;
mod spacing;
mod layout;
mod units;

use background::*;
use sizing::*;
use spacing::*;
use layout::*;

use serde::*;

#[derive(derive_more::Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Style {
    pub background_attachment: Option<BackgroundAttachment>,
    pub background_blend_mode: Option<BackgroundBlendMode>,
    pub background_clip: Option<BackgroundClip>,
    pub background_color: Option<BackgroundColor>,
    pub background_image: Option<BackgroundImage>,
    pub background_origin: Option<BackgroundOrigin>,
    pub background_position: Option<BackgroundPosition>,
    pub background_repeat: Option<BackgroundRepeat>,
    pub background_size: Option<BackgroundSize>,

    pub align_content: Option<AlignContent>,
    pub align_items: Option<AlignItems>,
    pub align_self: Option<AlignSelf>,

    pub box_decoration: Option<BoxDecoration>,
    pub box_sizing: Option<BoxSizing>,

    pub break_after: Option<BreakAfter>,
    pub break_before: Option<BreakBefore>,
    pub break_inside: Option<BreakInside>,

    pub flex_basis: Option<FlexBasis>,
    pub flex_direction: Option<FlexDirection>,
    pub flex_grow: Option<FlexGrow>,
    pub flex_shrink: Option<FlexShrink>,
    pub flex_wrap: Option<FlexWrap>,
    // pub flex: Option<Flex>,

    pub grid_auto_columns: Option<GridAutoColumns>,
    pub grid_auto_flow: Option<GridAutoFlow>,
    pub grid_auto_rows: Option<GridAutoRows>,
    pub grid_column: Option<GridColumn>,
    pub grid_row: Option<GridRow>,
    pub grid_template_columns: Option<GridTemplateColumns>,
    pub grid_template_rows: Option<GridTemplateRows>,

    pub justify_content: Option<JustifyContent>,
    pub justify_items: Option<JustifyItems>,
    pub justify_self: Option<JustifySelf>,

    pub object_fit: Option<ObjectFit>,
    pub object_position: Option<ObjectPosition>,

    pub place_content: Option<PlaceContent>,
    pub place_items: Option<PlaceItems>,
    pub place_self: Option<PlaceSelf>,

    pub position: Option<Position>,
    pub left: Option<Left>,
    pub right: Option<Right>,
    pub top: Option<Top>,
    pub bottom: Option<Bottom>,

    pub aspect_ratio: Option<AspectRatio>,
    pub clear: Option<Clear>,
    pub columns: Option<Columns>,
    pub display: Option<Display>,
    pub float: Option<Float>,
    pub gap: Option<Gap>,
    pub order: Option<Order>,
    pub overflow: Option<Overflow>,
    pub overscroll_behavior: Option<OverscrollBehavior>,
    pub visibility: Option<Visibility>,
    pub z_index: Option<ZIndex>,

    pub height: Option<Height>,
    pub width: Option<Width>,
    pub max_height: Option<MaxHeight>,
    pub max_width: Option<MaxWidth>,
    pub min_height: Option<MinHeight>,
    pub min_width: Option<MinWidth>,

    pub padding: Option<Padding>,
    pub margin: Option<Margin>,
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        [
            self.background_attachment.as_ref().map(|v| v.to_string()),
            self.background_blend_mode.as_ref().map(|v| v.to_string()),
            self.background_clip.as_ref().map(|v| v.to_string()),
            self.background_color.as_ref().map(|v| v.to_string()),
            self.background_image.as_ref().map(|v| v.to_string()),
            self.background_origin.as_ref().map(|v| v.to_string()),
            self.background_position.as_ref().map(|v| v.to_string()),
            self.background_repeat.as_ref().map(|v| v.to_string()),
            self.background_size.as_ref().map(|v| v.to_string()),

            self.align_content.as_ref().map(|v| v.to_string()),
            self.align_items.as_ref().map(|v| v.to_string()),
            self.align_self.as_ref().map(|v| v.to_string()),

            self.box_decoration.as_ref().map(|v| v.to_string()),
            self.box_sizing.as_ref().map(|v| v.to_string()),

            self.break_after.as_ref().map(|v| v.to_string()),
            self.break_before.as_ref().map(|v| v.to_string()),
            self.break_inside.as_ref().map(|v| v.to_string()),

            self.flex_basis.as_ref().map(|v| v.to_string()),
            self.flex_direction.as_ref().map(|v| v.to_string()),
            self.flex_grow.as_ref().map(|v| v.to_string()),
            self.flex_shrink.as_ref().map(|v| v.to_string()),
            self.flex_wrap.as_ref().map(|v| v.to_string()),

            self.grid_auto_columns.as_ref().map(|v| v.to_string()),
            self.grid_auto_flow.as_ref().map(|v| v.to_string()),
            self.grid_auto_rows.as_ref().map(|v| v.to_string()),
            self.grid_column.as_ref().map(|v| v.to_string()),
            self.grid_row.as_ref().map(|v| v.to_string()),
            self.grid_template_columns.as_ref().map(|v| v.to_string()),
            self.grid_template_rows.as_ref().map(|v| v.to_string()),

            self.justify_content.as_ref().map(|v| v.to_string()),
            self.justify_items.as_ref().map(|v| v.to_string()),
            self.justify_self.as_ref().map(|v| v.to_string()),

            self.object_fit.as_ref().map(|v| v.to_string()),
            self.object_position.as_ref().map(|v| v.to_string()),

            self.place_content.as_ref().map(|v| v.to_string()),
            self.place_items.as_ref().map(|v| v.to_string()),
            self.place_self.as_ref().map(|v| v.to_string()),

            self.position.as_ref().map(|v| v.to_string()),
            self.left.as_ref().map(|v| v.to_string()),
            self.right.as_ref().map(|v| v.to_string()),
            self.top.as_ref().map(|v| v.to_string()),
            self.bottom.as_ref().map(|v| v.to_string()),

            self.aspect_ratio.as_ref().map(|v| v.to_string()),
            self.clear.as_ref().map(|v| v.to_string()),
            self.columns.as_ref().map(|v| v.to_string()),
            self.display.as_ref().map(|v| v.to_string()),
            self.float.as_ref().map(|v| v.to_string()),
            self.gap.as_ref().map(|v| v.to_string()),
            self.order.as_ref().map(|v| v.to_string()),
            self.overflow.as_ref().map(|v| v.to_string()),
            self.overscroll_behavior.as_ref().map(|v| v.to_string()),
            self.visibility.as_ref().map(|v| v.to_string()),
            self.z_index.as_ref().map(|v| v.to_string()),
        ]
        .into_iter()
        .flatten()
        .map(|s| format!("{};", s))
        .collect::<Vec<_>>()
        .join(" ")
        .fmt(f)

    }
}

