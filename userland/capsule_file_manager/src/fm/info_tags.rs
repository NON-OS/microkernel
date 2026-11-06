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
use nonos_toolkit::widgets::{Button, Kind, Pill, Rect};

use super::chip::{chip_w, CHIP_H};
use super::info_geom::InfoGeom;
use super::info_sizes::SEC_PX;
use super::info_tag_add::{add_box, ADD_LABEL};
use super::info_tag_geom::{remove_x, tag_slots, TagSlot, X_BOX};
use super::state::State;
use super::target::Target;
use super::target_at::look;

const HEADING: &str = "Tags";

// The icon set has no close mark, so the cross is stroked here from the same
// anti-aliased line primitive `icon_draw` uses, doubled on one axis for the
// weight every other glyph in the chrome carries.
const ARM: i32 = 5;

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

/// One tag as a pill, then its remove box: a `Plain` button body that shows
/// hover and press, with the cross over it.
fn tag_chip(state: &State, fb: &mut PaintBuffer, index: usize, slot: &TagSlot) {
    Pill::new(slot.name).draw(fb, Rect { x: slot.x, y: slot.y, w: chip_w(slot.name), h: CHIP_H });
    let (x, y) = (remove_x(slot), slot.y);
    let body = Button::new("").kind(Kind::Plain).state(look(state, Target::Untag(index)));
    body.draw(fb, Rect { x, y, w: X_BOX, h: CHIP_H });
    let px = x as i32;
    let py = (y + CHIP_H.saturating_sub(X_BOX) / 2) as i32;
    let (far, ink) = (X_BOX as i32 - ARM, roles().muted.argb());
    fb.line_aa(px + ARM, py + ARM, px + far, py + far, ink);
    fb.line_aa(px + ARM + 1, py + ARM, px + far + 1, py + far, ink);
    fb.line_aa(px + far, py + ARM, px + ARM, py + far, ink);
    fb.line_aa(px + far - 1, py + ARM, px + ARM - 1, py + far, ink);
}
