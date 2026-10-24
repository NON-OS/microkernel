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

//! Where a rectangle lands fitted: exactly what the mirror's resample
//! writes, and, the other way, a screen's rectangle on the smaller canvas
//! it was scaled from, grown outward so nothing it shows is left out.

use crate::state::damage::Rect;
use crate::sw_blitter::{fitted, resample, Surface};

fn t(r: Rect) -> (u32, u32, u32, u32) {
    (r.x, r.y, r.width, r.height)
}

#[test]
fn fitted_is_what_resample_writes() {
    let mut src = vec![0x11u32; 64 * 48];
    let mut dst = vec![0u32; 50 * 50];
    let s = Surface {
        base_va: src.as_mut_ptr() as u64,
        stride: 256,
        width: 64,
        height: 48,
        byte_len: 64 * 48 * 4,
    };
    let d = Surface {
        base_va: dst.as_mut_ptr() as u64,
        stride: 200,
        width: 50,
        height: 50,
        byte_len: 50 * 50 * 4,
    };
    for (x, y, w, h) in [(0, 0, 64, 48), (3, 5, 7, 9), (60, 40, 10, 10), (63, 47, 1, 1)] {
        let r = Rect { x, y, width: w, height: h };
        let want = fitted((64, 48), (50, 50), r).map(t);
        assert_eq!(resample(s, d, r).map(t), want, "({x}, {y}) {w} x {h}");
    }
    assert!(fitted((64, 48), (50, 50), Rect { x: 64, y: 0, width: 4, height: 4 }).is_none());
}

#[test]
fn a_screen_rectangle_on_its_canvas_covers_what_it_shows() {
    let r = Rect { x: 101, y: 33, width: 9, height: 3 };
    let c = fitted((1920, 1080), (960, 540), r).map(t);
    assert_eq!(c, Some((50, 16, 5, 2)), "halved, grown to whole pixels");
    assert_eq!(fitted((1920, 1080), (1920, 1080), r).map(t), Some(t(r)), "the same size: itself");
}
