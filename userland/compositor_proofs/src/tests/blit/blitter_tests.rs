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

//! The blitter writes raw pixels through pointers, so the property that matters
//! is containment: a fill or a composite must never touch a byte outside the
//! destination surface, no matter how the source rectangle, clip, or layer
//! placement is chosen. We back a real `Surface` with a guarded heap buffer and
//! prove the guard bytes are untouched, plus that clipping is exact.

use super::blitter_canvas::*;
use crate::damage::Rect;
use crate::sw_blitter::{composite_layer, fill_rect};

#[test]
fn fill_stays_inside_the_surface_when_rect_overflows() {
    let mut c = Canvas::new();
    let s = c.surface();
    // A rect that runs off the right and bottom edges must be clipped, not wrap.
    fill_rect(s, Rect { x: 30, y: 25, width: 100, height: 100 }, 0xFFFF_0000);
    c.assert_guards_intact();
    assert_eq!(c.pixel(39, 29), 0xFFFF_0000, "bottom-right corner should be filled");
    assert_eq!(c.pixel(0, 0), 0, "far corner should be untouched");
}

#[test]
fn composite_clips_to_both_surface_and_clip_rect() {
    let mut c = Canvas::new();
    let s = c.surface();
    let (_keep, src) = opaque_src(20, 20, 0xFF00_FF00);
    // Place the layer straddling the edge, and clip to a small window.
    composite_layer(s, src, 30, 20, 20, 20, Rect { x: 32, y: 22, width: 4, height: 4 });
    c.assert_guards_intact();
    // Only the clipped 4x4 window should have been written.
    assert_eq!(c.pixel(32, 22), 0xFF00_FF00);
    assert_eq!(c.pixel(35, 25), 0xFF00_FF00);
    assert_eq!(c.pixel(36, 26), 0, "outside the clip must stay untouched");
    assert_eq!(c.pixel(31, 21), 0, "outside the clip must stay untouched");
}

#[test]
fn composite_skips_transparent_source_pixels() {
    let mut c = Canvas::new();
    let s = c.surface();
    fill_rect(s, Rect { x: 0, y: 0, width: W, height: H }, 0xFF20_2020);
    let (_keep, src) = opaque_src(8, 8, 0x0000_0000); // alpha 0 everywhere
    composite_layer(s, src, 4, 4, 8, 8, Rect { x: 0, y: 0, width: W, height: H });
    c.assert_guards_intact();
    assert_eq!(c.pixel(6, 6), 0xFF20_2020, "transparent source must not overwrite the background");
}

#[test]
fn fill_fully_off_screen_is_a_noop() {
    let mut c = Canvas::new();
    let s = c.surface();
    fill_rect(s, Rect { x: W + 5, y: 0, width: 4, height: 4 }, 0xFFAA_AAAA);
    c.assert_guards_intact();
    assert_eq!(c.pixel(0, 0), 0);
}
