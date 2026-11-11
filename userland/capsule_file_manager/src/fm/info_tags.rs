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
use nonos_toolkit::widgets::{Button, Kind, Pill, Rect};

use super::chip::{chip_w, CHIP_H};
use super::header_slots::CROSS_S;
use super::info_geom::InfoGeom;
use super::info_sizes::SEC_PX;
use super::info_tag_add::{add_box, ADD_LABEL};
use super::info_tag_geom::{remove_x, tag_slots, TagSlot, X_BOX};
use super::state::State;
use super::target::Target;
use super::target_at::look;

const HEADING: &str = "Tags";

/// The tags on the cursor path as a wrapped band of shared `Pill`s, each
/// followed by the box that removes it, and an add pill closing the band.
/// Chips wrap when the next measured one would overrun `g.w`; `info_tag_geom`
/// decides where each one lands and `info_tag_hit` reads the same pass back.
pub fn tag_band(state: &State, fb: &mut PaintBuffer, g: &InfoGeom, names: &[&str]) {
    if g.chips_y + CHIP_H > g.bottom {
        return;
    }
    let _ = fb.text_ttf(g.x as i32, g.tags_head as i32, HEADING, roles().muted.argb(), SEC_PX);
    let slots = tag_slots(names, g.x, g.chips_y, g.w);
    for (i, slot) in slots.iter().enumerate() {
        tag_chip(state, fb, i, slot);
    }
    let (ax, ay, aw) = add_box(&slots, g.x, g.chips_y, g.w);
    let add = Pill::new(ADD_LABEL).state(look(state, Target::AddTag));
    add.draw(fb, Rect { x: ax, y: ay, w: aw, h: CHIP_H });
}

/// One tag as a pill, then its remove box: a `Quiet` button body that shows
/// hover and press and carries the cross.
fn tag_chip(state: &State, fb: &mut PaintBuffer, index: usize, slot: &TagSlot) {
    Pill::new(slot.name).draw(fb, Rect { x: slot.x, y: slot.y, w: chip_w(slot.name), h: CHIP_H });
    let (x, y) = (remove_x(slot), slot.y);
    let body = Button::new("").kind(Kind::Quiet).icon(IconId::Close).icon_size(CROSS_S);
    body.state(look(state, Target::Untag(index))).draw(fb, Rect { x, y, w: X_BOX, h: CHIP_H });
}
