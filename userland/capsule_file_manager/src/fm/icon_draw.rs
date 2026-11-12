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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::Opaque;
use nonos_toolkit::widgets::draw as paint;

use super::icon_path::{Mark, Pt, CUBE};
use super::layout::hair;

/// Draws `m` into the `size`-square box at `(x, y)` in `ink`: a toolkit mark
/// through the shared icon painter, the cube through the stroker below. Both
/// blend, so this is safe over paint the capsule already laid down.
pub fn draw(fb: &mut PaintBuffer, m: Mark, x: u32, y: u32, size: u32, ink: Opaque) {
    match m {
        Mark::Kit(id) => paint::icon(fb, id, x, y, size, ink),
        Mark::Cube => cube(fb, x, y, size, ink.argb()),
    }
}

fn cube(fb: &mut PaintBuffer, x: u32, y: u32, size: u32, color: u32) {
    for poly in CUBE {
        let mut prev: Option<(i32, i32)> = None;
        for p in poly.iter() {
            let cur = map(x, y, size, *p);
            if let Some(a) = prev {
                stroke(fb, a, cur, color);
            }
            prev = Some(cur);
        }
    }
}

fn map(x: u32, y: u32, size: u32, p: Pt) -> (i32, i32) {
    let s = size as i32;
    (x as i32 + (p.0 as i32 * s) / 1000, y as i32 + (p.1 as i32 * s) / 1000)
}

fn stroke(fb: &mut PaintBuffer, a: (i32, i32), b: (i32, i32), color: u32) {
    fb.line_aa(a.0, a.1, b.0, b.1, color);
    let (ox, oy) = if (b.0 - a.0).abs() >= (b.1 - a.1).abs() { (0, 1) } else { (1, 0) };
    for k in 1..(hair() * 2) as i32 {
        fb.line_aa(a.0 + ox * k, a.1 + oy * k, b.0 + ox * k, b.1 + oy * k, color);
    }
}
