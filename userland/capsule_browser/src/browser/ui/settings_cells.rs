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

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::CONTROL_H;

use super::rect::Rect;
use super::settings_rows::{Control, Field, Item, Kind, Line};

pub const GAP: u32 = 8;
const HINT_H: u32 = 28;
const BTN_W: u32 = 88;
const SIZE_W: u32 = 240;

/// `n` logical pixels at the display scale.
pub fn px(n: u32) -> i32 {
    scale().px(n) as i32
}

/// The height of every control on the page: a toolkit button's.
pub fn ctl_h() -> i32 {
    px(CONTROL_H)
}

pub fn push_ctl(out: &mut Vec<Item>, c: Control, rect: Rect) {
    out.push(Item { kind: Kind::Ctl(c), rect });
}

/// A field filling the band `ctl` left of its buttons `cs`, with the line
/// `hint` under it.
pub fn field_row(out: &mut Vec<Item>, ctl: Rect, f: Field, cs: &[Control], hint: Line) {
    let (n, gap, h) = (cs.len() as i32, px(GAP), ctl_h());
    let bw = px(BTN_W).min(ctl.w / (n + 2));
    let fw = (ctl.w - n * (bw + gap)).max(0);
    push_ctl(out, Control::Field(f), Rect::new(ctl.x, ctl.y, fw, h));
    for (i, c) in cs.iter().enumerate() {
        let x = ctl.x + fw + gap + i as i32 * (bw + gap);
        push_ctl(out, *c, Rect::new(x, ctl.y, bw, h));
    }
    let line = Rect::new(ctl.x, ctl.bottom() + px(2), ctl.w, px(HINT_H));
    out.push(Item { kind: Kind::Line(hint), rect: line });
}

/// Smaller and Larger at either end of the band `ctl`, the size dots
/// between them.
pub fn size_row(out: &mut Vec<Item>, ctl: Rect) {
    let (h, gap) = (ctl_h(), px(GAP));
    let span = px(SIZE_W).min(ctl.w).max(2 * h);
    let dots_w = (span - 2 * (h + gap)).max(0);
    push_ctl(out, Control::Smaller, Rect::new(ctl.x, ctl.y, h, h));
    let dots = Rect::new(ctl.x + h + gap, ctl.y, dots_w, h);
    out.push(Item { kind: Kind::Line(Line::SizeDots), rect: dots });
    push_ctl(out, Control::Larger, Rect::new(ctl.x + span - h, ctl.y, h, h));
}
