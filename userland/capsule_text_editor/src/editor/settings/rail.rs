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

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Controls, NavItem};

use super::geom::{nav_row, rail_pad, rail_w, rail_x, title_px, title_top, NAV_LABELS};
use super::pick::Pick;

const TITLE_INSET: u32 = 12;

pub(super) fn paint_rail(fb: &mut PaintBuffer, selected: usize, c: &mut Controls<Pick>) {
    let (h, r, hair) = (fb.height, roles(), scale().hair());
    fb.fill_rect(rail_x(), 0, rail_w(), h, r.sunken.argb());
    fb.fill_rect(rail_x() + rail_w() - hair, 0, hair, h, r.border.argb());
    let tx = (rail_x() + rail_pad() + scale().px(TITLE_INSET)) as i32;
    let _ = fb.text_ttf(tx, title_top() as i32, "Docs Settings", r.text.argb(), title_px());
    for (i, label) in NAV_LABELS.iter().enumerate() {
        let at = nav_row(i);
        let item = NavItem::new(label).active(i == selected);
        item.state(c.place(at, Pick::Nav(i))).draw(fb, at);
    }
}
