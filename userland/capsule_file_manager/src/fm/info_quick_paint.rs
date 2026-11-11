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
use nonos_toolkit::widgets::{Button, Kind, Rect};

use super::header_slots::strip;
use super::icon_draw::draw;
use super::info_geom::InfoGeom;
use super::info_quick::{quick_slots, Quick};
use super::info_sizes::{QUICK_H, SEC_PX};
use super::layout::{font, px};
use super::state::State;
use super::target::Target;
use super::target_at::look;

const HEADING: &str = "Quick Actions";

/// The tile row under the panel head, drawn only when the panel is tall enough
/// to hold it. The hit-test applies the same clip, so a tile that was skipped is
/// never clickable. Each tile is a shared `Button` body, Delete a `Danger` one.
pub fn paint_quick(state: &State, fb: &mut PaintBuffer, g: &InfoGeom) {
    let (quick_h, glyph) = (px(QUICK_H), strip().glyph);
    if g.quick_y + quick_h > g.bottom {
        return;
    }
    let r = roles();
    let _ = fb.text_ttf(g.x as i32, g.quick_head as i32, HEADING, r.muted.argb(), font(SEC_PX));
    let gy = g.quick_y + quick_h.saturating_sub(glyph) / 2;
    for (i, slot) in quick_slots(g.x, g.w).iter().enumerate() {
        let danger = slot.action == Quick::Delete;
        let kind = if danger { Kind::Danger } else { Kind::Secondary };
        let at = Rect { x: slot.x, y: g.quick_y, w: slot.w, h: quick_h };
        Button::new("").kind(kind).state(look(state, Target::Quick(i))).draw(fb, at);
        let ink = if danger { r.danger } else { r.muted }.argb();
        let gx = slot.x + slot.w.saturating_sub(glyph) / 2;
        draw(fb, slot.icon, gx, gy, glyph, ink);
    }
}
