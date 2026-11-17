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
use nonos_toolkit::widgets::text::mono_left;
use nonos_toolkit::widgets::{NavItem, Rect, Separator, State as Look};

use crate::pm::format::u32_decimal;
use crate::pm::state::{Screen, State, SCREENS};

use super::fit::{rail, sidebar_w};
use super::hit::Target;
use super::hover_look::pointed;
use super::icon_table::icon;
use super::metrics::metrics;
use super::nav_geom::{row_w, row_x, row_y};
use super::text;

/// The sidebar down the window's left side, `h` tall; each entry draws the
/// pointer's hover and press.
pub fn paint(fb: &mut PaintBuffer, state: &State, h: u32) {
    let (r, w, hair) = (roles(), sidebar_w(fb.width), scale().hair());
    fb.fill_rect(0, 0, w, h, r.sunken.argb());
    fb.fill_rect(w - hair, 0, hair, h, r.border.argb());
    for (i, screen) in SCREENS.iter().enumerate() {
        let look = pointed(state, |t| t == Target::Nav(*screen));
        paint_row(fb, *screen, row_y(i), *screen == state.screen, look);
    }
    /* Folded to its icons there is no room for the sampling line. */
    if !rail(fb.width) {
        paint_foot(fb, h, state.refreshes);
    }
}

/// One entry: the toolkit item over the whole row box `nav_geom` hit-tests,
/// drawing its own icon and label inside it. Folded, the label is left out.
fn paint_row(fb: &mut PaintBuffer, screen: Screen, y: u32, active: bool, look: Look) {
    let label = if rail(fb.width) { "" } else { text::valid(screen.nav_label()) };
    let at = Rect { x: row_x(), y, w: row_w(fb.width), h: metrics().nav_h };
    NavItem::new(label).icon(icon(screen)).active(active).state(look).draw(fb, at);
}

// Provenance: the refresh counter advances once a second, which is the cheapest
// on-screen proof that the whole window is live data and not a frozen frame.
fn paint_foot(fb: &mut PaintBuffer, h: u32, refreshes: u32) {
    let (r, m) = (roles(), metrics());
    let (y, px) = (h.saturating_sub(m.side_foot_h), m.body_px);
    Separator::new().draw(fb, Rect { x: row_x(), y, w: row_w(fb.width), h: scale().hair() });
    let x = row_x() + m.nav_pad_x;
    text::left(fb, x, y + m.side_foot_top, b"SAMPLED", r.muted, px);
    let mut buf = [0u8; 12];
    let n = u32_decimal(refreshes, &mut buf);
    let line = y + m.side_foot_top + m.side_foot_line;
    let advance = text::left(fb, x, line, b"1.0s ", r.muted, px);
    mono_left(fb, advance.max(0) as u32, line as i32, text::valid(&buf[..n]), r.muted, px);
}
