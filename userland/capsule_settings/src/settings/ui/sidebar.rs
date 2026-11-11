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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{NavItem, Rect, State as Look};

use crate::settings::section::{Section, SECTIONS};
use crate::settings::section_text::nav;
use crate::settings::state::State;

use super::hover_look::pointed;
use super::icon_table::icon;
use super::metrics::metrics;
use super::nav_geom::{row_w, row_x, row_y};
use super::target::Target;

/// The sidebar down the window's left side, `state.win_h` tall; each entry
/// draws the pointer's hover and press.
pub fn paint(fb: &mut PaintBuffer, state: &State) {
    let (r, h, w, hair) = (roles(), state.win_h, metrics().sidebar_w, scale().hair());
    fb.fill_rect(0, 0, w, h, r.sunken.argb());
    fb.fill_rect(w - hair, 0, hair, h, r.border.argb());
    for (i, section) in SECTIONS.iter().enumerate() {
        let look = pointed(state, |t| t == Target::Nav(*section));
        paint_row(fb, *section, row_y(i), *section == state.section, look);
    }
}

/// One entry: the toolkit item over the whole row box, drawing its own icon
/// and label inside it, so the active wash covers both.
fn paint_row(fb: &mut PaintBuffer, section: Section, y: u32, active: bool, look: Look) {
    let at = Rect { x: row_x(), y, w: row_w(), h: metrics().nav_h };
    NavItem::new(nav(section)).icon(icon(section)).active(active).state(look).draw(fb, at);
}
