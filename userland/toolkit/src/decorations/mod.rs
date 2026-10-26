// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! The frame's measures at one to one (HANDOFF/ui/identity.md, Window
mod accessory;
mod control_rect;
mod controls;
mod edges;
mod frame_rect;
mod glyph;
mod hit_test;
mod inner_rect;
mod metrics;
mod palette;
mod rect;
mod reshape;
mod scale;
mod title;
mod titlebar;
mod window_for;

pub use accessory::{accessory_rect, accessory_rect_at};
pub use control_rect::{control_rect, control_rect_at, controls_left_at, CONTROLS};
pub use controls::draw_controls_at;
pub use edges::{edges_at, Edges, EDGE_PX, TOP_EDGE_PX};
pub use frame_rect::{
    border_at, frame_rect, frame_rect_at, margin, margin_at, radius, radius_at, titlebar_h_at,
};
pub use hit_test::{hit_test, hit_test_at, DecorationHit};
pub use inner_rect::{
    chrome_growth_at, content_rect, content_rect_at, titlebar_rect, titlebar_rect_at,
};
pub use metrics::{
    ACCESSORY_INSET, ACCESSORY_PAD_Y, BORDER_PX, CONTROL_H, CONTROL_INSET, CONTROL_W, FRAME_RADIUS,
    GLYPH_ARM, GLYPH_STROKE_Q, HAIRLINE_PX, SHADOW_MARGIN, TITLEBAR_H, TITLE_INSET, TITLE_PX,
};
pub use palette::{
    control_wash, focus_rule, frame_bg, frame_edge, glyph_hover, glyph_rest, title_ink,
    CLOSE_HOVER, ON_CLOSE_HOVER, SHADOW, TRANSPARENT,
};
pub use rect::Rect;
pub use reshape::{keep_far_edges, reshaped, Placed};
pub use scale::{at as scaled, px_at as scaled_px, ONE as SCALE_ONE};
pub use titlebar::{draw_focus_rule_at, draw_frame, draw_frame_at, FrameState};
pub use window_for::window_for;
