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

//! Where a composite lands and what it writes: a layer placed off the
//! screen writes nothing, and a translucent one blends over what is there.

use super::blitter_canvas::*;
use crate::damage::Rect;
use crate::sw_blitter::{composite_layer, fill_rect};

#[test]
fn composite_off_screen_placement_writes_nothing() {
    let mut c = Canvas::new();
    let s = c.surface();
    let (_keep, src) = opaque_src(10, 10, 0xFFFF_FFFF);
    // Entirely past the right edge.
    composite_layer(s, src, 200, 5, 10, 10, Rect { x: 0, y: 0, width: W, height: H });
    c.assert_guards_intact();
    for y in 0..H {
        for x in 0..W {
            assert_eq!(c.pixel(x, y), 0, "nothing should have been drawn");
        }
    }
}

#[test]
fn composite_blends_a_translucent_source_over_the_destination() {
    let mut c = Canvas::new();
    let s = c.surface();
    // Destination is solid black; source is 50% white. The result should be a
    // mid grey, proving the layer is blended rather than overwritten.
    fill_rect(s, Rect { x: 0, y: 0, width: W, height: H }, 0xFF00_0000);
    let (_keep, src) = opaque_src(4, 4, 0x80FF_FFFF); // alpha 0x80, white
    composite_layer(s, src, 2, 2, 4, 4, Rect { x: 0, y: 0, width: W, height: H });
    c.assert_guards_intact();
    let p = c.pixel(3, 3);
    let ch = |sh: u32| (p >> sh) & 0xFF;
    assert_eq!(p >> 24, 0xFF, "result stays opaque");
    // 0xFF * 128 / 255 ~= 128; allow a rounding margin.
    for sh in [0u32, 8, 16] {
        assert!((120..=136).contains(&ch(sh)), "channel {sh} not blended: {:#010x}", p);
    }
}
