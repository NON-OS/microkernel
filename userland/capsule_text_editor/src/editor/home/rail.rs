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

//! The left navigation rail: the brand over a navigation item per list, each
//! laid out in the screen's controls as it is drawn. The account block
//! that sat at its foot named a person ("Mehedi Hasan, Local Account") the
//! machine knows nothing of; the editor has no accounts, so it is gone.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Controls, NavItem};

use super::brand::paint_brand;
use super::metrics::{nav_row, rail_w, rail_x};
use super::pick::HomePick;
use super::state::NAV_LABELS;

pub(super) fn paint_rail(fb: &mut PaintBuffer, selected: usize, c: &mut Controls<HomePick>) {
    let (h, r, hair) = (fb.height, roles(), scale().hair());
    let inner = rail_w() - hair;
    fb.fill_rect(rail_x(), 0, inner, h, r.sunken.argb());
    fb.fill_rect(rail_x() + inner, 0, hair, h, r.border.argb());
    paint_brand(fb);
    for (i, label) in NAV_LABELS.iter().enumerate() {
        let at = nav_row(i);
        let item = NavItem::new(label).active(i == selected);
        item.state(c.place(at, HomePick::Nav(i))).draw(fb, at);
    }
}
