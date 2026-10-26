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
//! The frame's measures at one to one (HANDOFF/ui/identity.md, Window
//! The controls' line glyphs: a 1.5 px stroke in a 16 px box at one to one,
//! growing with the display scale. A stroke that is not a whole number of
//! pixels draws its last row or column at the remaining coverage, so 1.5 px
//! reads as 1.5 px rather than as 1 or 2.

use super::hit_test::DecorationHit;
use super::metrics::{GLYPH_ARM, GLYPH_STROKE_Q};
use super::scale::{at, ONE};
use crate::paint::PaintBuffer;

/// The stroke at `q`, in quarters of a pixel.
fn stroke_q(q: u32) -> u32 {
    GLYPH_STROKE_Q * q.max(ONE) / 4
}

fn faded(argb: u32, quarters: u32) -> u32 {
    ((((argb >> 24) & 0xFF) * quarters / 4) << 24) | (argb & 0x00FF_FFFF)
}

/// A run `len` long from (x, y), `s` quarters thick, across or down.
fn bar(fb: &mut PaintBuffer, x: u32, y: u32, len: u32, s: u32, down: bool, argb: u32) {
    let (whole, part) = (s / 4, s % 4);
    for k in 0..whole + (part != 0) as u32 {
        let c = if k < whole { argb } else { faded(argb, part) };
        let (px, py, w, h) = if down { (x + k, y, 1, len) } else { (x, y + k, len, 1) };
        fb.blend_rect(px, py, w, h, c);
    }
}

/// Control `which`'s glyph centred on (cx, cy).
pub(super) fn draw(fb: &mut PaintBuffer, which: DecorationHit, cx: u32, cy: u32, q: u32, c: u32) {
    let (a, s) = (at(GLYPH_ARM, q), stroke_q(q));
    let (x0, y0, len) = (cx.saturating_sub(a), cy.saturating_sub(a), a * 2);
    let t = s.div_ceil(4);
    match which {
        DecorationHit::MinimizeButton => bar(fb, x0, cy, len, s, false, c),
        DecorationHit::MaximizeButton => {
            bar(fb, x0, y0, len, s, false, c);
            bar(fb, x0, y0 + len - t, len, s, false, c);
            bar(fb, x0, y0, len, s, true, c);
            bar(fb, x0 + len - t, y0, len, s, true, c);
        }
        DecorationHit::CloseButton => {
            // A diagonal's run across is its stroke by root two.
            let n = (s * 141 / 100).div_ceil(4).max(1) as i32;
            let (l, r, top, bot) = (x0 as i32, (x0 + len) as i32, y0 as i32, (y0 + len) as i32);
            for k in 0..n {
                let o = k - n / 2;
                fb.line_aa(l + o, top, r + o, bot, c);
                fb.line_aa(r + o, top, l + o, bot, c);
            }
        }
        _ => {}
    }
}
