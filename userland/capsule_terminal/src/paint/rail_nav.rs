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

//! The rail's two lists atop its scrolled column: a `SectionHeader` and a
//! plus `Button` heading each, then a `Row` per session or pinned path, in
//! rail-local content pixels before the scroll, for painter, hover and click.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::{CONTROL_H, SPACE};
use nonos_toolkit::widgets::{Rect, Row, SectionHeader};

/// Each list's heading, its plus button and the band its rows fill.
pub struct Nav {
    pub s_head: Rect,
    pub s_plus: Rect,
    pub s_list: Rect,
    pub p_head: Rect,
    pub p_plus: Rect,
    pub p_list: Rect,
}

/// A list row: a name with its path as the note.
pub fn row_h() -> u32 {
    Row::new("").note("").size().1
}

/// A heading: the header's lines with `SPACE.sm` above and below.
pub fn head_h() -> u32 {
    SectionHeader::new("").size().1.saturating_add(scale().px(u32::from(SPACE.sm)) * 2)
}

/// The two lists stacked `w` wide from the column's top, `SPACE.sm` apart.
/// Every row is laid out: the column scrolls rather than the lists cutting.
pub fn nav(w: u32, sessions: u32, projects: u32) -> Nav {
    let (hh, rh, gap) = (head_h(), row_h(), scale().px(u32::from(SPACE.sm)));
    let s_head = Rect { x: 0, y: 0, w, h: hh };
    let s_list = Rect { y: hh, h: sessions * rh, ..s_head };
    let p_head = Rect { y: s_list.y + s_list.h + gap, ..s_head };
    let p_list = Rect { y: p_head.y + hh, h: projects * rh, ..s_head };
    Nav { s_plus: plus(s_head), s_head, s_list, p_plus: plus(p_head), p_head, p_list }
}

/// What the two lists take out of the column.
pub fn nav_h(sessions: u32, projects: u32) -> u32 {
    nav(0, sessions, projects).p_list.y + projects * row_h()
}

/// A `CONTROL_H` square `SPACE.sm` in from the heading's right, centred down it.
fn plus(head: Rect) -> Rect {
    let s = scale();
    let side = s.px(CONTROL_H).min(head.h);
    let x = head.w.saturating_sub(s.px(u32::from(SPACE.sm)) + side);
    Rect { x, y: head.y + (head.h - side) / 2, w: side, h: side }
}

/// The heading's title box: `SPACE.lg` in, as a row pads its label, to the plus.
pub fn title(head: Rect, plus: Rect) -> Rect {
    let s = scale();
    let x = s.px(u32::from(SPACE.lg));
    Rect { x, w: plus.x.saturating_sub(x + s.px(u32::from(SPACE.sm))), ..head }
}
