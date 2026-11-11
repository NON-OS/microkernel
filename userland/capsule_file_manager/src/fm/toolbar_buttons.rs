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
use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Button, Card, Kind, Rect, State as Look};

use super::header_slots::{Slot, GLYPH_S, TOOL_H};
use super::icon_draw::draw;
use super::icon_path::Icon;
use super::state::State;
use super::target::Target;
use super::target_at::look;

/// Vertical origin that centres a `size`-square glyph in the control band.
pub fn glyph_y(y: u32, size: u32) -> u32 {
    y + TOOL_H.saturating_sub(size) / 2
}

/// The shared card under the slots from `first` to `last` inclusive: the
/// ground a group of adjacent controls shares, so the group reads as one track
/// rather than as neighbouring buttons.
pub fn group_plate(fb: &mut PaintBuffer, first: &Slot, last: &Slot, y: u32) {
    let w = (last.x + last.w).saturating_sub(first.x);
    Card::new().draw(fb, Rect { x: first.x, y, w, h: TOOL_H });
}

/// A `Plain` button body over the slot, which shows the hover and press of
/// target `t` and draws nothing at rest, for a control whose marks are drawn
/// over it.
pub fn plain(state: &State, fb: &mut PaintBuffer, slot: &Slot, y: u32, t: Target) {
    let body = Button::new("").kind(Kind::Plain).state(look(state, t));
    body.draw(fb, Rect { x: slot.x, y, w: slot.w, h: TOOL_H });
}

/// A `Quiet` button body over the slot for header target `slot.hit`, carrying
/// `mark`, a toolkit icon and its side, laid out and inked by the button.
pub fn plain_icon(state: &State, fb: &mut PaintBuffer, slot: &Slot, y: u32, mark: (IconId, u32)) {
    let body = Button::new("").kind(Kind::Quiet).icon(mark.0).icon_size(mark.1);
    let lit = look(state, Target::Head(slot.hit));
    body.state(lit).draw(fb, Rect { x: slot.x, y, w: slot.w, h: TOOL_H });
}

/// A bare toolkit icon on an inset `Quiet` body, drawn onto a ground someone
/// else laid down, laid out and inked by the button. A dimmed button has
/// nothing to act on yet.
pub fn glyph_btn(state: &State, fb: &mut PaintBuffer, slot: &Slot, y: u32, id: IconId, dim: bool) {
    let lit = Look { disabled: dim, ..look(state, Target::Head(slot.hit)) };
    let body = Button::new("").kind(Kind::Quiet).state(lit).icon(id).icon_size(GLYPH_S);
    body.draw(fb, inset(slot, y));
}

/// A bare icon control drawn onto a ground someone else laid down. The active
/// state raises its own inset `Secondary` body, which is what makes a segmented
/// toggle read as one track with a lit segment.
pub fn icon_btn(state: &State, fb: &mut PaintBuffer, slot: &Slot, y: u32, icon: Icon, on: bool) {
    let r = roles();
    let kind = if on { Kind::Secondary } else { Kind::Plain };
    Button::new("").kind(kind).state(look(state, Target::Head(slot.hit))).draw(fb, inset(slot, y));
    let ink = if on { r.accent } else { r.muted };
    let gx = slot.x + slot.w.saturating_sub(GLYPH_S) / 2;
    draw(fb, icon, gx, glyph_y(y, GLYPH_S), GLYPH_S, ink.argb());
}

fn inset(slot: &Slot, y: u32) -> Rect {
    Rect { x: slot.x + 2, y: y + 2, w: slot.w.saturating_sub(4), h: TOOL_H - 4 }
}
