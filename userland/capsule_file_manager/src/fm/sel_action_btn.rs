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
use nonos_toolkit::widgets::{text, Button, Kind, Rect};

use super::header_slots::{GLYPH_GAP, GLYPH_S, ICON_PAD, TOOL_H, TOOL_PX};
use super::icon_draw::draw;
use super::sel_model::SelAction;
use super::sel_slots::SelSlot;
use super::state::State;
use super::target::Target;
use super::target_at::look;

/// One action in the band, the `index`th, on a shared `Button` body. Delete is
/// the one destructive entry and the only one that spends the danger hue, as a
/// `Danger` button.
pub fn action(
    state: &State,
    fb: &mut PaintBuffer,
    index: usize,
    slot: &SelSlot,
    y: u32,
    labelled: bool,
) {
    let danger = slot.action == SelAction::Delete;
    let kind = if danger { Kind::Danger } else { Kind::Secondary };
    let at = Rect { x: slot.x, y, w: slot.w, h: TOOL_H };
    Button::new("").kind(kind).state(look(state, Target::Band(index))).draw(fb, at);
    let r = roles();
    let ink = if danger { r.danger } else { r.muted };
    let gy = y + TOOL_H.saturating_sub(GLYPH_S) / 2;
    if !labelled {
        let gx = slot.x + slot.w.saturating_sub(GLYPH_S) / 2;
        draw(fb, slot.icon, gx, gy, GLYPH_S, ink.argb());
        return;
    }
    draw(fb, slot.icon, slot.x + ICON_PAD, gy, GLYPH_S, ink.argb());
    let pen = slot.x + ICON_PAD + GLYPH_S + GLYPH_GAP;
    text::left(fb, pen, text::centre_top(y, TOOL_H, TOOL_PX), slot.label, ink, TOOL_PX);
}
