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

//! What a page shows with nothing in it: the player's glyph on a disc, and a
//! heading and a note under it as shared Labels.

use nonos_app_skeleton::paint::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::text_role::TextRole;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::rect::centre;
use nonos_toolkit::widgets::{draw, Align, Ink, Label, Rect};

use crate::ui::marks::Glyph;

const DISC: u32 = 76;
const ICON: u32 = 34;
const GAP: u32 = 20;
const HEAD_H: u32 = 26;
const NOTE_H: u32 = 24;

pub fn paint_empty(fb: &mut PaintBuffer, r: Rect, glyph: Glyph, head: &str, note: &str) {
    let s = scale();
    let (d, gap, head_h, note_h) = (s.px(DISC), s.px(GAP), s.px(HEAD_H), s.px(NOTE_H));
    let top = r.y + r.h.saturating_sub(d + gap + head_h + note_h) / 2;
    let (cx, theme) = (r.x + r.w / 2, roles());
    let disc = Rect { x: cx.saturating_sub(d / 2), y: top, w: d, h: d };
    draw::round(fb, disc, d / 2, theme.surface);
    draw::outline(fb, disc, d / 2, s.hair(), theme.border);
    let mark = centre(disc, s.px(ICON), s.px(ICON));
    glyph(fb, mark.x, mark.y, mark.w, theme.quiet.argb());
    let room = r.w.saturating_sub(s.px(48)).min(s.px(420));
    let line = |y: u32, h: u32| Rect { x: cx.saturating_sub(room / 2), y, w: room, h };
    let y = top + d + gap;
    let head_line = Label::new(head).role(TextRole::Title).align(Align::Centre);
    head_line.draw(fb, line(y, head_h));
    let note_line = Label::new(note).ink(Ink::Quiet).align(Align::Centre);
    note_line.draw(fb, line(y + s.px(30), note_h));
}
