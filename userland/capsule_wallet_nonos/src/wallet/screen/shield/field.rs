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

/*
 * A one-line text field: its name above, the typed text or a hint inside,
 * and a caret only in the field that has the keyboard. Text too long for the
 * field shows its tail, which is the part being typed.
 */

use nonos_app_skeleton::PaintBuffer;

use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::text::{draw_in, line, width};
use crate::wallet::etna::tokens::{CORNER, CYAN, FIELD, OUTLINE, TEXT, TEXT_3, TIGHT};
use crate::wallet::etna::Role;

const FIELD_H: u32 = 44;
const PAD: i32 = 12;

fn tail(text: &str, room: i32) -> &str {
    let start = super::typed::tail_start(text, room, |t| width(Role::RowValue, t));
    text.get(start..).unwrap_or("")
}

/* Returns the field's box, for the click table, and the height used. */
pub fn field(
    fb: &mut PaintBuffer,
    c: Rect,
    y: u32,
    name: &str,
    text: &str,
    hint: &str,
    focused: bool,
) -> (Rect, u32) {
    draw_in(fb, c.x as i32, y as i32, Role::Fact, name, TEXT_3);
    let top = y + line(Role::Fact) as u32 + TIGHT / 2;
    let at = Rect::new(c.x, top, c.w, FIELD_H);
    fb.fill_round(at.x, at.y, at.w, at.h, CORNER, FIELD);
    fb.stroke_round(at.x, at.y, at.w, at.h, CORNER, 1, if focused { CYAN } else { OUTLINE });
    let ty = at.y as i32 + (FIELD_H as i32 - line(Role::RowValue)) / 2;
    let room = at.w as i32 - 3 * PAD;
    let shown = if text.is_empty() { hint } else { tail(text, room) };
    let ink = if text.is_empty() { TEXT_3 } else { TEXT };
    let end = draw_in(fb, at.x as i32 + PAD, ty, Role::RowValue, shown, ink);
    if focused {
        let cx = if text.is_empty() { at.x as i32 + PAD } else { end + 1 };
        fb.fill_rect(cx as u32, ty as u32, 2, line(Role::RowValue) as u32, CYAN);
    }
    (at, top + FIELD_H - y)
}
