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

//! Where the logo goes on the splash's surface. The record is in the
//! firmware mode's pixels; the surface is the compositor's canvas, which can
//! be another mode or a scaled one. The logo keeps its centre's place in the
//! screen on each axis and is sized by the smaller of the two axis ratios,
//! so it is never stretched. With the mode equal to the canvas it lands on
//! the very pixels the firmware used.

/// A rectangle on the surface; it may run past an edge, and drawing clips.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spot {
    pub x: i64,
    pub y: i64,
    pub w: u32,
    pub h: u32,
}

/// The firmware mode when the kernel would not say: firmware centres its
/// logo across the screen (common practice, unverified as a rule), so the
/// width is twice the left margin plus the logo, and the height keeps the
/// canvas's shape, at least tall enough to hold the logo.
pub fn guess_mode(x: u32, y: u32, dw: u32, dh: u32, surface: (u32, u32)) -> (u32, u32) {
    let fw = 2 * x as u64 + dw as u64;
    let fh = (fw * surface.1 as u64).div_ceil(surface.0.max(1) as u64);
    (fw as u32, fh.max(y as u64 + dh as u64) as u32)
}

/// The logo of shown size `dw` by `dh` at (`x`, `y`) in mode `fw`, onto a
/// surface; None when the mode cannot hold it or it would shrink to nothing.
pub fn place(
    x: u32,
    y: u32,
    dw: u32,
    dh: u32,
    fw: (u32, u32),
    surface: (u32, u32),
) -> Option<Spot> {
    let (x, y, dw, dh) = (x as u64, y as u64, dw as u64, dh as u64);
    let (mw, mh, sw, sh) = (fw.0 as u64, fw.1 as u64, surface.0 as u64, surface.1 as u64);
    if mw == 0 || mh == 0 || x + dw > mw || y + dh > mh {
        return None;
    }
    let (num, den) = if sw * mh <= sh * mw { (sw, mw) } else { (sh, mh) };
    let (w, h) = (dw * num / den, dh * num / den);
    if w == 0 || h == 0 {
        return None;
    }
    let cx = (2 * x + dw) * sw / (2 * mw);
    let cy = (2 * y + dh) * sh / (2 * mh);
    Some(Spot {
        x: cx as i64 - (w / 2) as i64,
        y: cy as i64 - (h / 2) as i64,
        w: w as u32,
        h: h as u32,
    })
}
