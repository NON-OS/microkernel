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

//! The canvas enlarged onto the screen by a fractional step (1.25, 1.5):
//! `q` quarters of a screen pixel per canvas pixel, each screen pixel the
//! canvas pixel its centre falls in. Whole steps go through `upscale`.

use super::Surface;
use crate::state::damage::Rect;

/// The column table: `n` entries, or None when there is no memory for it.
pub type Table = fn(usize) -> Option<alloc::vec::Vec<u32>>;

/// Copy `rect` of `canvas` onto `screen` at `q`/4 times its position and
/// size. Returns the screen rectangle written, every pixel whose centre
/// lies in `rect`, or None when none of it lands on the screen.
pub fn stretch(canvas: Surface, screen: Surface, rect: Rect, q: u32) -> Option<Rect> {
    stretch_with(canvas, screen, rect, q, room)
}

/// `stretch`, its column table from `table`: a table that cannot be had
/// leaves each pixel to work its column out.
pub fn stretch_with(
    canvas: Surface,
    screen: Surface,
    rect: Rect,
    q: u32,
    table: Table,
) -> Option<Rect> {
    if q < 4 || rect.x >= canvas.width || rect.y >= canvas.height {
        return None;
    }
    let q = q as u64;
    let x1 = (rect.x as u64 + rect.width as u64).min(canvas.width as u64);
    let y1 = (rect.y as u64 + rect.height as u64).min(canvas.height as u64);
    // Screen pixel s shows canvas pixel (4s + 2) / q, the one its centre
    // falls in; the first screen pixel of canvas pixel c is (cq + 1) / 4.
    let first = |c: u64| (c * q + 1) / 4;
    let (sx0, sy0) = (first(rect.x as u64), first(rect.y as u64));
    let sx1 = first(x1).min(screen.width as u64);
    let sy1 = first(y1).min(screen.height as u64);
    if sx0 >= sx1 || sy0 >= sy1 {
        return None;
    }
    let src = |s: u64| ((4 * s + 2) / q) as u32;
    let n = (sx1 - sx0) as u32;
    // The canvas column of each screen column, once for the rectangle.
    let last = canvas.width.saturating_sub(1);
    let cols = table(n as usize).map(|mut v| {
        v.extend((0..n as usize).map(|i| src(sx0 + i as u64).min(last)));
        v
    });
    let mut built: Option<(u32, usize)> = None;
    for sy in sy0..sy1 {
        let dst_va = screen.row_start(sy as u32, sx0 as u32, n)?;
        // SAFETY: `row_start` bounds checked the screen row for `n` pixels.
        let dst = unsafe { core::slice::from_raw_parts_mut(dst_va as *mut u32, n as usize) };
        // A screen row showing the canvas row the last one showed is a copy.
        if let Some((_, va)) = built.filter(|&(c, _)| c == src(sy)) {
            // SAFETY: an earlier screen row of this rectangle, bounds checked
            // for `n` pixels when it was built, and not this one.
            dst.copy_from_slice(unsafe {
                core::slice::from_raw_parts(va as *const u32, n as usize)
            });
            continue;
        }
        let row = canvas.row_start(src(sy), 0, canvas.width)?;
        // SAFETY: `row_start` bounds checked the canvas row whole; the canvas
        // is never the screen when it is enlarged onto it.
        let line = unsafe { core::slice::from_raw_parts(row as *const u32, canvas.width as usize) };
        match &cols {
            Some(cols) => dst.iter_mut().zip(cols).for_each(|(px, &c)| *px = line[c as usize]),
            None => {
                for (i, px) in dst.iter_mut().enumerate() {
                    *px = line[(src(sx0 + i as u64) as usize).min(line.len() - 1)];
                }
            }
        }
        built = Some((src(sy), dst_va));
    }
    Some(Rect { x: sx0 as u32, y: sy0 as u32, width: n, height: (sy1 - sy0) as u32 })
}

/// Room for `n` columns, or None when there is no memory for the table.
fn room(n: usize) -> Option<alloc::vec::Vec<u32>> {
    let mut v = alloc::vec::Vec::new();
    v.try_reserve_exact(n).ok()?;
    Some(v)
}
