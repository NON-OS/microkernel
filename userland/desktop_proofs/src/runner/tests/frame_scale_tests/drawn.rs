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
//! A window's frame at a scale: its measures, its pixels and its title's rows.
//! What is drawn is where the geometry says: glyphs with no colour at rest,
//! the hovered control's wash, close's red, and the bar's rule.

use super::rig::{paint, paint_in, px, AT_REST, H, SCALES, W};
use nonos_toolkit::decorations::{
    content_rect_at, control_rect_at, frame_bg, frame_edge, glyph_rest, DecorationHit, FrameState,
    Rect, CLOSE_HOVER,
};

fn at(px: &[u32], x: u32, y: u32) -> u32 {
    px[(y * W + x) as usize]
}

#[test]
fn what_is_drawn_is_where_the_geometry_says() {
    for q in SCALES {
        let px = paint(q, b"");
        let (min, max) = (control_rect_at(0, W, H, false, q), control_rect_at(1, W, H, false, q));
        let centre = |r: Rect| (r.x + r.w / 2, r.y + r.h / 2);
        let (mx, my) = centre(min);
        assert_eq!(at(&px, mx, my), glyph_rest(), "minimize's line through its centre at {q}");
        let (xx, xy) = centre(max);
        assert_eq!(at(&px, xx, xy), frame_bg(), "maximize's square is open at {q}");
        assert_eq!(at(&px, min.x + 1, min.y + min.h / 2), frame_bg(), "no wash at rest");
        let c = content_rect_at(W, H, false, q);
        assert_eq!(at(&px, W / 2, c.y), frame_edge(), "the bar's rule at {q}");
    }
}

#[test]
fn the_hovered_control_is_washed_and_close_is_red() {
    for q in SCALES {
        let close = control_rect_at(2, W, H, false, q);
        let hot = paint_in(q, b"", FrameState { hover: DecorationHit::CloseButton, ..AT_REST });
        assert_eq!(at(&hot, close.x + px(8, q), close.y + close.h / 2), CLOSE_HOVER, "{q}");
        let min = control_rect_at(0, W, H, false, q);
        let lit = paint_in(q, b"", FrameState { hover: DecorationHit::MinimizeButton, ..AT_REST });
        let (x, y) = (min.x + px(4, q), min.y + min.h / 2);
        assert_ne!(at(&lit, x, y), frame_bg(), "minimize washed at {q}");
        assert_eq!(at(&lit, close.x + px(8, q), close.y + close.h / 2), frame_bg());
    }
}
