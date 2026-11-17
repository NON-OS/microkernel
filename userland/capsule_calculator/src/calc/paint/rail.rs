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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Card, NavItem, Rect};

use crate::calc::event::hover::look;
use crate::calc::hit::Hit;
use crate::calc::mode::MODES;
use crate::calc::state::State;
use crate::calc::ui::icon_table::icon;
use crate::calc::ui::metrics::metrics;
use crate::calc::ui::nav_geom::{row, row_x};

const BRAND: &str = "Calculator";

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let h = fb.height;
    let (m, r) = (metrics(), roles());
    let card_h = (h as i32 - m.pane_pad * 2).max(0) as u32;
    let (x, y) = (m.rail_x as u32, m.pane_pad as u32);
    Card::new().draw(fb, Rect { x, y, w: m.rail_card_w as u32, h: card_h });
    let top = text::centre_top(m.brand_top as u32, m.brand_h as u32, m.px_title);
    text::left(fb, row_x() as u32, top, BRAND, r.accent, m.px_title);
    for (i, mode) in MODES.iter().enumerate() {
        let item = NavItem::new(mode.label()).icon(icon(*mode)).active(state.mode == *mode);
        item.state(look(state, Hit::Rail(*mode))).draw(fb, row(i));
    }
}
