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

//! Where a popup goes, as xdg-shell places it, an axis at a time
//! (axis.rs), within the bounds it is kept to. Pure, so the host proofs
//! hold it. Anything still outside is clipped where it is drawn.

use super::axis::{fit, Axis};
use super::positioner::{Positioner, FLIP_X, FLIP_Y, RESIZE_X, RESIZE_Y, SLIDE_X, SLIDE_Y};
use super::side::sides;

fn axis(start: i32, len: i32, (anchor, gravity): (i8, i8), (size, offset): (i32, i32)) -> Axis {
    let (start, len, size, offset) = (start.into(), len.into(), size.into(), offset.into());
    Axis { start, len, anchor, gravity, size, offset }
}

/// (x, y, width, height) of the popup `p` places within `bounds`, both on
/// the parent's surface.
pub fn place(p: &Positioner, (bx, by, bw, bh): (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    let ((ax, ay), (gx, gy)) = (sides(p.anchor), sides(p.gravity));
    let x = axis(p.rect.0, p.rect.2, (ax, gx), (p.size.0, p.offset.0));
    let y = axis(p.rect.1, p.rect.3, (ay, gy), (p.size.1, p.offset.1));
    let on = |bit: u32| p.adjust & bit != 0;
    let span = |at: i32, len: i32| (i64::from(at), i64::from(at) + i64::from(len));
    let (x, w) = fit(x, span(bx, bw), (on(FLIP_X), on(SLIDE_X), on(RESIZE_X)));
    let (y, h) = fit(y, span(by, bh), (on(FLIP_Y), on(SLIDE_Y), on(RESIZE_Y)));
    let i = |v: i64| v.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
    (i(x), i(y), i(w), i(h))
}
