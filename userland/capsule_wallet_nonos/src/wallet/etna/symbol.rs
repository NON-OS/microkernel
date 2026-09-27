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

//! The few signs the wallet draws, as two-pixel strokes about a centre, in
//! place of the phones' SF Symbols of the same names.

use nonos_app_skeleton::PaintBuffer;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    ArrowUp,
    ArrowDown,
    ArrowLeftRight,
    LockShield,
    ArrowDownToLine,
    ChevronLeft,
}

fn stroke(fb: &mut PaintBuffer, cx: i32, cy: i32, segs: &[(i32, i32, i32, i32)], argb: u32) {
    for &(x0, y0, x1, y1) in segs {
        for d in 0..2 {
            fb.line(cx + x0 + d, cy + y0, cx + x1 + d, cy + y1, argb);
            fb.line(cx + x0, cy + y0 + d, cx + x1, cy + y1 + d, argb);
        }
    }
}

pub fn symbol(fb: &mut PaintBuffer, cx: i32, cy: i32, which: Symbol, argb: u32) {
    let segs: &[(i32, i32, i32, i32)] = match which {
        Symbol::ArrowUp => &[(0, -8, 0, 8), (-6, -2, 0, -8), (6, -2, 0, -8)],
        Symbol::ArrowDown => &[(0, -8, 0, 8), (-6, 2, 0, 8), (6, 2, 0, 8)],
        Symbol::ArrowDownToLine => &[(0, -8, 0, 4), (-5, -1, 0, 4), (5, -1, 0, 4), (-7, 8, 7, 8)],
        Symbol::ArrowLeftRight => &[(-8, -3, 8, -3), (4, -7, 8, -3), (8, 3, -8, 3), (-4, 7, -8, 3)],
        Symbol::LockShield => &[
            (-7, -7, 0, -9),
            (0, -9, 7, -7),
            (-7, -7, -6, 2),
            (7, -7, 6, 2),
            (-6, 2, 0, 8),
            (6, 2, 0, 8),
            (-3, -1, 3, -1),
            (-3, -1, -3, 4),
            (3, -1, 3, 4),
            (-3, 4, 3, 4),
        ],
        Symbol::ChevronLeft => &[(3, -7, -4, 0), (-4, 0, 3, 7)],
    };
    stroke(fb, cx, cy, segs, argb);
}

/// The way to settings: a toothed ring, drawn centred in `at`.
pub fn gear(fb: &mut PaintBuffer, at: super::rect::Rect) {
    let (cx, cy) = (at.x + at.w / 2, at.y + at.h / 2);
    fb.ring(cx, cy, 7, 2, super::tokens::TEXT);
    for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)] {
        let (x0, y0) = (cx as i32 + dx * 7, cy as i32 + dy * 7);
        fb.line(x0, y0, x0 + dx * 3, y0 + dy * 3, super::tokens::TEXT);
    }
}
