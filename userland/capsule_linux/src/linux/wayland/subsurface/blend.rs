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

//! One subsurface's pixels laid on the window's frame. Pure, so the host
//! proofs hold it.
//!
//! Both are ARGB8888 as Wayland has it: B, G, R, A in memory, colour
//! premultiplied by alpha, and an XRGB buffer already made opaque as it was
//! read (opaque.rs). Over is Porter-Duff's (`nonos_pixel_rows::scalar::over`):
//! what is laid on top keeps its colour, and what is beneath shows through
//! as much as it is transparent.
//! A layer beneath the window is laid under the frame, which is the same
//! sum with the two swapped, so a window drawn bottom first or top first
//! comes out the same.

/// A frame, or a layer: `w` by `h` pixels, `stride` bytes a row.
#[derive(Clone, Copy)]
pub struct Plane {
    pub w: u32,
    pub h: u32,
    pub stride: u32,
}

/// Lay `layer`, shaped `ls`, at (`x`, `y`) of `frame`, shaped `fs`: on top
/// of it when `over`, else beneath it. What falls outside the frame, or
/// past the end of either slice, is left out.
pub fn lay(frame: &mut [u8], fs: Plane, layer: &[u8], ls: Plane, (x, y): (i32, i32), over: bool) {
    let (x, y) = (i64::from(x), i64::from(y));
    let left = x.max(0);
    let right = (x + i64::from(ls.w)).min(i64::from(fs.w));
    let top = y.max(0);
    let bottom = (y + i64::from(ls.h)).min(i64::from(fs.h));
    if right <= left {
        return;
    }
    let span = ((right - left) * 4) as usize;
    for row in top..bottom {
        let f = (row * i64::from(fs.stride) + left * 4) as usize;
        let l = ((row - y) * i64::from(ls.stride) + (left - x) * 4) as usize;
        let (Some(d), Some(s)) = (frame.get_mut(f..), layer.get(l..)) else {
            continue;
        };
        // Whole pixels both sides still hold: the old per-pixel test, once.
        let n = span.min(d.len() / 4 * 4).min(s.len() / 4 * 4);
        if over {
            nonos_pixel_rows::over_row(&mut d[..n], &s[..n]);
        } else {
            nonos_pixel_rows::under_row(&mut d[..n], &s[..n]);
        }
    }
}
