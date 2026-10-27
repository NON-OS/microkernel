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

//! A surface's scale, and the sizes and points it is shown at, as ratios of
//! shown to logical pixels in 120ths, as wp_fractional_scale_v1 says a
//! scale. A window is its buffer's own pixels while the client draws at the
//! display's scale or below it; a finer buffer is brought down to it, and a
//! viewport's destination is shown at it. Pure, so the host proofs hold it.

pub const ONE: u32 = 120;

/// set_buffer_scale's factor, and the surface's viewport and its destination.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Scale {
    pub buffer: u32,
    pub dest: Option<(u32, u32)>,
    pub viewport: u32,
}

/// The display's ratio from its scale in quarters (4 is 1:1).
pub fn display(quarters: u32) -> u32 {
    quarters.max(1) * 30
}

/// The whole scale wl_output and wl_surface name: the display's, rounded down.
pub fn whole(quarters: u32) -> u32 {
    (quarters / 4).max(1)
}

/// The ratio a window is shown at, for its toplevel's `s` on display `d`.
pub fn window_ratio(s: Scale, d: u32) -> u32 {
    match s.dest {
        Some(_) => d,
        None => ONE.saturating_mul(s.buffer.max(1)).min(d),
    }
}

/// The size a `w` by `h` buffer of a surface with `s` is shown at, at
/// ratio `e`: its logical size, the destination or the buffer over its
/// scale, times `e`, to the nearest pixel.
pub fn shown(s: Scale, (w, h): (u32, u32), e: u32) -> (u32, u32) {
    let by = ONE.saturating_mul(s.buffer.max(1));
    let (w, h, by) = s.dest.map_or((w, h, by), |(dw, dh)| (dw, dh, ONE));
    let at = |v: u32| (u64::from(v) * u64::from(e) + u64::from(by) / 2) / u64::from(by);
    let at = |v: u32| at(v).clamp(1, u64::from(u32::MAX)) as u32;
    (at(w), at(h))
}

/// A logical point as a shown one at ratio `e`, and a shown coordinate
/// back as a logical one.
pub fn point_px((x, y): (i32, i32), e: u32) -> (i32, i32) {
    (times(x, e, ONE), times(y, e, ONE))
}

pub fn to_logical(v: i32, e: u32) -> i32 {
    times(v, ONE, e)
}

fn times(v: i32, by: u32, over: u32) -> i32 {
    let v = (i64::from(v) * i64::from(by)).div_euclid(i64::from(over.max(1)));
    v.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}
