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

use super::header_slots::{
    Slot, GLYPH_GAP, GLYPH_S, ICON_PAD, NEW_LABEL, TOOL_H, TOOL_PX,
};
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::state::State;
use super::target::Target;
use super::target_at::look;

/// A button that leads with a glyph and follows it with a label, laid out from
/// the same padding `icon_pill_w` measured the slot with. The body is the shared
/// `Button` of `kind`; the glyph and label are drawn over it in its ink.
pub fn icon_label(
    state: &State,
    fb: &mut PaintBuffer,
    slot: &Slot,
    y: u32,
    icon: Icon,
    label: &str,
) {
    labelled(state, fb, slot, y, (icon, label), Kind::Secondary);
}

/// The one accented control in the header: a `Primary` button that starts a
/// new file, which is what marks it as the row's primary action.
pub fn new_btn(state: &State, fb: &mut PaintBuffer, slot: &Slot, y: u32) {
    labelled(state, fb, slot, y, (Icon::Plus, NEW_LABEL), Kind::Primary);
}

fn labelled(
    state: &State,
    fb: &mut PaintBuffer,
    slot: &Slot,
    y: u32,
    mark: (Icon, &str),
    kind: Kind,
) {
    let at = Rect { x: slot.x, y, w: slot.w, h: TOOL_H };
    Button::new("").kind(kind).state(look(state, Target::Head(slot.hit))).draw(fb, at);
    let r = roles();
    let ink = if kind == Kind::Primary { r.on_accent } else { r.muted };
    let gy = y + TOOL_H.saturating_sub(GLYPH_S) / 2;
    draw(fb, mark.0, slot.x + ICON_PAD, gy, GLYPH_S, ink.argb());
    let pen = slot.x + ICON_PAD + GLYPH_S + GLYPH_GAP;
    text::left(fb, pen, text::centre_top(y, TOOL_H, TOOL_PX), mark.1, ink, TOOL_PX);
}
