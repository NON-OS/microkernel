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

//! A buffer taken again brought up to the picture: the part of it out of
//! date (its stale box) copied from the buffer shown, less what the draw
//! about to run writes over anyway. Pure, so the host proofs hold it.

/// A box in buffer pixels: x, y, width, height.
pub type Area = (u32, u32, u32, u32);

/// `a` cut to a `w` by `h` buffer; None when nothing of it is on it.
pub fn clip((x, y, bw, bh): Area, (w, h): (u32, u32)) -> Option<Area> {
    let (x1, y1) = (x.saturating_add(bw).min(w), y.saturating_add(bh).min(h));
    (x < x1 && y < y1).then(|| (x, y, x1 - x, y1 - y))
}

/// What of `stale` lies outside `cover`: the rows above it and below it,
/// then the columns left and right of it in the rows they share. Each pixel
/// of `stale` not in `cover` is in exactly one of them, and none is in both.
pub fn outside(stale: Area, cover: Area) -> [Option<Area>; 4] {
    let (sx, sy, sw, sh) = stale;
    let (sx1, sy1) = (sx.saturating_add(sw), sy.saturating_add(sh));
    let (cx, cy, cw, ch) = cover;
    let (cx1, cy1) = (cx.saturating_add(cw), cy.saturating_add(ch));
    if sw == 0 || sh == 0 {
        return [None; 4];
    }
    if cw == 0 || ch == 0 || cx >= sx1 || cx1 <= sx || cy >= sy1 || cy1 <= sy {
        return [Some(stale), None, None, None];
    }
    let span = |x0: u32, y0: u32, x1: u32, y1: u32| {
        (x0 < x1 && y0 < y1).then(|| (x0, y0, x1 - x0, y1 - y0))
    };
    let (my0, my1) = (sy.max(cy), sy1.min(cy1));
    [
        span(sx, sy, sx1, my0),
        span(sx, my1, sx1, sy1),
        span(sx, my0, sx.max(cx), my1),
        span(sx1.min(cx1), my0, sx1, my1),
    ]
}

/// Box `a`, cut to a `w` by `h` buffer, copied from `from` into `to`, both
/// `stride` bytes a row. False, with what fits copied, when a row of it runs
/// past either slice, which two buffers of that shape never do.
pub fn copy(to: &mut [u8], from: &[u8], (w, h, stride): (u32, u32, u32), a: Area) -> bool {
    let Some((x, y, bw, bh)) = clip(a, (w, h)) else { return true };
    let (stride, x0, n) = (stride as usize, x as usize * 4, bw as usize * 4);
    for row in y as usize..(y + bh) as usize {
        let at = row * stride + x0;
        let (Some(dst), Some(src)) = (to.get_mut(at..at + n), from.get(at..at + n)) else {
            return false;
        };
        dst.copy_from_slice(src);
    }
    true
}
