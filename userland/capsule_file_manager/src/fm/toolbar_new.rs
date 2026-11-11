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
use nonos_toolkit::widgets::{Button, Kind, Rect};

use super::header_slots::{strip, Slot, GLYPH_S, NEW_LABEL};
use super::state::State;
use super::target::Target;
use super::target_at::look;

/// A button that leads with a glyph and follows it with a label, the pair
/// laid out and inked by the shared `Button` of `kind` that `icon_pill_w`
/// measured the slot with.
pub fn icon_label(
    state: &State,
    fb: &mut PaintBuffer,
    slot: &Slot,
    y: u32,
    icon: IconId,
    label: &str,
) {
    labelled(state, fb, slot, y, (icon, label), Kind::Secondary);
}

/// The one accented control in the header: a `Primary` button that starts a
/// new file, which is what marks it as the row's primary action.
pub fn new_btn(state: &State, fb: &mut PaintBuffer, slot: &Slot, y: u32) {
    labelled(state, fb, slot, y, (IconId::Plus, NEW_LABEL), Kind::Primary);
}

fn labelled(
    state: &State,
    fb: &mut PaintBuffer,
    slot: &Slot,
    y: u32,
    mark: (IconId, &str),
    kind: Kind,
) {
    let at = Rect { x: slot.x, y, w: slot.w, h: strip().h };
    let body = Button::new(mark.1).kind(kind).icon(mark.0).icon_size(GLYPH_S);
    body.state(look(state, Target::Head(slot.hit))).draw(fb, at);
}
