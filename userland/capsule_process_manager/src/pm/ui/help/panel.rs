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

//! The overlay itself.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::SPACE;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::rect::inset;
use nonos_toolkit::widgets::{Card, Ink, Label, Rect};

use super::super::keys_group::Group;
use super::super::keys_table::all;
use super::super::text;
use super::geom::{column, panel, GROUP_GAP, ROW_H};
use super::row;

// Which groups fall in which column, left first.
const COLUMNS: [&[Group]; 2] = [&[Group::Move, Group::Sort], &[Group::Filter, Group::Act]];

/// The window behind stays readable through the theme's scrim rather than
/// being hidden. A reader opening this is checking a key against what is on
/// screen, so blanking the table would make them close the panel to remember
/// what they were doing. The panel is a toolkit card over the scrim, its head
/// the title and how to close it.
pub fn paint(fb: &mut PaintBuffer, fb_w: u32, fb_h: u32) {
    let r = roles();
    fb.blend_rect(0, 0, fb_w, fb_h, r.scrim.argb());
    let (x, y, w, h) = panel(fb_w, fb_h);
    let (at, card) = (Rect { x, y, w, h }, Card::new().title("KEYBOARD"));
    let card = card.note("? or Esc closes this");
    card.draw(fb, at);
    for (i, groups) in COLUMNS.iter().enumerate() {
        render(fb, column(card.content(at), i as u32), groups);
    }
}

// One column: each group under its heading, in table order.
fn render(fb: &mut PaintBuffer, at: Rect, groups: &[Group]) {
    let (s, mut y) = (scale(), at.y);
    let (pad, h) = (s.px(u32::from(SPACE.lg)), s.px(ROW_H));
    for group in groups {
        let head = Rect { y, h, ..inset(at, pad) };
        Label::new(text::valid(group.label())).ink(Ink::Muted).draw(fb, head);
        y += h;
        for b in all().filter(|b| b.group == *group) {
            row::paint(fb, Rect { y, h, ..at }, b.key, b.label);
            y += h;
        }
        y += s.px(GROUP_GAP);
    }
}
