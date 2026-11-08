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

//! The left navigation rail: the screen title over a navigation item per
//! section, each laid out in the screen's controls as it is drawn.

use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Controls, NavItem};

use super::geom::{nav_row, title_top, NAV_LABELS, RAIL_PAD, RAIL_W, RAIL_X, TITLE_PX};
use super::pick::Pick;

pub(super) fn paint_rail(fb: &mut PaintBuffer, selected: usize, c: &mut Controls<Pick>) {
    let (h, r) = (fb.height, roles());
    fb.fill_rect(RAIL_X, 0, RAIL_W, h, r.sunken.argb());
    fb.fill_rect(RAIL_X + RAIL_W - 1, 0, 1, h, r.border.argb());
    let tx = (RAIL_X + RAIL_PAD + 12) as i32;
    let _ = fb.text_ttf(tx, title_top() as i32, "Docs Settings", r.text.argb(), TITLE_PX);
    for (i, label) in NAV_LABELS.iter().enumerate() {
        let at = nav_row(i);
        let item = NavItem::new(label).active(i == selected);
        item.state(c.place(at, Pick::Nav(i))).draw(fb, at);
    }
}
