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
use nonos_toolkit::widgets::{text, NavItem, Rect, Separator};

use super::icon_draw::draw;
use super::screen::Screen;
use super::sidebar_icon::nav_icon;
use super::sidebar_metrics::{line_box, rail_text_x, rail_w, ICON_S, RAIL_PAD, RAIL_X};
use super::sidebar_model::{SideHit, SideRow};
use super::state::State;
use super::target::Target;
use super::target_at::look;

const LABEL_PX: f32 = 18.0;
const PILL_INSET: u32 = 4;

/// One navigation row: a `NavItem` body, which draws the active row's accent
/// and the hover and press of the rest. The icon set the rail wears is the
/// manager's own, so the glyph and the label after it are drawn here, in the
/// ink the body chose for them.
pub fn nav_row(state: &State, fb: &mut PaintBuffer, index: usize, row: &SideRow) {
    let hit = match row.hit.as_ref() {
        Some(hit) => hit,
        None => return,
    };
    let active = row_active(state, hit);
    let h = row.h.saturating_sub(PILL_INSET);
    let at = Rect { x: RAIL_X, y: row.y, w: rail_w(), h };
    NavItem::new("").active(active).state(look(state, Target::Side(index))).draw(fb, at);
    let r = roles();
    let ink = if active { r.accent_text } else { r.muted };
    let iy = row.y + h.saturating_sub(ICON_S) / 2;
    draw(fb, nav_icon(hit), RAIL_X + RAIL_PAD, iy, ICON_S, ink.argb());
    let tx = rail_text_x();
    let avail = (RAIL_X + rail_w()).saturating_sub(tx + RAIL_PAD);
    let label = text::truncate(row.label.as_str(), avail, LABEL_PX);
    let ty = row.y + h.saturating_sub(line_box(LABEL_PX)) / 2;
    text::left(fb, tx, ty as i32, &label, ink, LABEL_PX);
}

/// Active when the row is the drawn surface, or Browse is in the path it names.
pub fn row_active(state: &State, hit: &SideHit) -> bool {
    match hit {
        SideHit::Screen(s) => state.screen == *s,
        SideHit::Path(p) => state.screen == Screen::Browse && state.prefix.as_str() == p.as_str(),
    }
}
/// The hairline between two groups, centred in the band the rule owns.
pub fn rule(fb: &mut PaintBuffer, y: u32, h: u32) {
    let x = RAIL_X + RAIL_PAD;
    let w = rail_w().saturating_sub(RAIL_PAD * 2);
    Separator::new().draw(fb, Rect { x, y: y + h / 2, w, h: 1 });
}
