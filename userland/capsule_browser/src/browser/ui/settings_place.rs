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

use alloc::vec::Vec;

use super::rect::Rect;
use super::settings_cells::{ctl_h, field_row, push_ctl, px, size_row, GAP};
use super::settings_rows::{Item, Kind};
use super::settings_table::Row;

const PAD: u32 = 16;
const ROW_H: u32 = 56;
const FIELD_ROW_H: u32 = 84;
const LABEL_W: u32 = 180;
const OPT_W: u32 = 132;
const ACT_W: u32 = 300;

/// Where the next row of a card goes.
pub struct Cursor {
    pub frame: Rect,
    pub y: i32,
}

/// Lays out `row` at the cursor, pushing its label and controls, and moves
/// the cursor below it. The label takes a column at the left, its rect the
/// card's edge to the column's end, a row tall; the controls share the band
/// right of it, a control tall. A labelled row after the first, `rule`, has
/// a hairline across the card above it.
pub fn place(at: &mut Cursor, row: &Row, rule: bool, out: &mut Vec<Item>) {
    let (pad, gap, ch, rh) = (px(PAD), px(GAP), ctl_h(), px(ROW_H));
    let h = if matches!(row, Row::Field(..)) { px(FIELD_ROW_H) } else { rh };
    let inner = Rect::new(at.frame.x + pad, at.y, (at.frame.w - 2 * pad).max(0), h);
    let lw = px(LABEL_W).min(inner.w / 3);
    let y = inner.y + (rh - ch) / 2;
    let ctl = Rect::new(inner.x + lw + gap, y, (inner.w - lw - gap).max(0), ch);
    let (Row::Options(s, _)
    | Row::Field(s, ..)
    | Row::Action(s, _)
    | Row::Text(s, _)
    | Row::Size(s)) = *row;
    if rule && !s.is_empty() {
        out.push(Item { kind: Kind::Rule, rect: Rect::new(inner.x, at.y, inner.w, 0) });
    }
    let label = Rect::new(at.frame.x, at.y, pad + lw, rh);
    out.push(Item { kind: Kind::Label(s), rect: label });
    match *row {
        Row::Options(_, cs) => {
            let n = cs.len() as i32;
            let w = px(OPT_W).min((ctl.w - (n - 1) * gap) / n).max(0);
            for (i, c) in cs.iter().enumerate() {
                push_ctl(out, *c, Rect::new(ctl.x + i as i32 * (w + gap), y, w, ch));
            }
        }
        Row::Field(_, f, cs, hint) => field_row(out, ctl, f, cs, hint),
        Row::Action(_, c) => push_ctl(out, c, Rect::new(ctl.x, y, px(ACT_W).min(ctl.w), ch)),
        Row::Text(_, l) => out.push(Item { kind: Kind::Line(l), rect: ctl }),
        Row::Size(_) => size_row(out, ctl),
    }
    at.y += h;
}
