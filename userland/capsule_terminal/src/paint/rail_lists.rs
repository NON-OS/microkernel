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
//! The rail's two lists: each heading a `SectionHeader` with a plus `Button`,
//! each entry a `Row` with its path as the note and a status dot in its
//! trailing box, the chosen one selected.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::DOT;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{
    draw, Button, Kind, Pointer, Rect, Row, SectionHeader, State, Surface,
};

use super::rail_column::Column;
use super::rail_nav::{nav, title};
use super::rail_spot::row;

/// One list entry: its name, the path under it, and whether it is busy.
pub struct Entry<'a> {
    pub name: &'a str,
    pub path: &'a str,
    pub busy: bool,
}

/// Paints both lists in content pixels; `fb` is the column scrolled by `c.off`.
pub fn draw_lists(
    fb: &mut impl Surface,
    c: &Column,
    ptr: &Pointer,
    sessions: &[Entry],
    projects: &[Entry],
    active: usize,
) {
    let n = nav(c.rect.w, sessions.len() as u32, projects.len() as u32);
    list(fb, c, ptr, (n.s_head, n.s_plus, n.s_list), "Sessions", sessions, Some(active));
    list(fb, c, ptr, (n.p_head, n.p_plus, n.p_list), "Projects", projects, None);
}

fn list(
    fb: &mut impl Surface,
    c: &Column,
    ptr: &Pointer,
    (head, plus, rows): (Rect, Rect, Rect),
    caption: &str,
    entries: &[Entry],
    chosen: Option<usize>,
) {
    SectionHeader::new(caption).draw(fb, title(head, plus));
    Button::new("+").kind(Kind::Plain).state(c.look(ptr, plus)).draw(fb, plus);
    let (r, side) = (roles(), scale().px(DOT));
    for (i, e) in entries.iter().enumerate() {
        let at = row(rows, i as u32);
        if !c.shows(at) {
            continue;
        }
        let st = State { selected: chosen == Some(i), ..c.look(ptr, at) };
        let it = Row::new(e.name).note(e.path).trailing(side).state(st);
        it.draw(fb, at);
        let b = it.trailing_rect(at);
        draw::dot(fb, b.x + b.w / 2, b.y + b.h / 2, side / 2, if e.busy { r.ok } else { r.muted });
    }
}
