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
use nonos_toolkit::widgets::{Button, Kind};

use crate::clock::controls::{keys, Target};
use crate::clock::fmt;
use crate::clock::hover::look;
use crate::clock::says::DONE;
use crate::clock::state::State;
use crate::clock::tabs::Tab;

/// Where the time being set and the note under the buttons sit, in logical
/// pixels, and their glyph sizes as multiples of the 8 pixel bitmap face at 1x.
const READOUT_X: u32 = 96;
const READOUT_Y: u32 = 100;
const READOUT_GLYPHS: u32 = 6;
const NOTE_X: u32 = 40;
const NOTE_Y: u32 = 366;
const NOTE_GLYPHS: u32 = 1;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let buf = fmt::hm(state.edit_hour, state.edit_min);
    let (r, s) = (roles(), scale());
    fb.text_scaled(s.px(READOUT_X), s.px(READOUT_Y), &buf, r.text.argb(), s.px(READOUT_GLYPHS));
    let k = keys(Tab::Set);
    for (i, name) in ["H-", "H+", "M-", "M+", "Apply"].into_iter().enumerate() {
        let kind = if i == 4 { Kind::Primary } else { Kind::Secondary };
        Button::new(name).kind(kind).state(look(state, Target::Key(i))).draw(fb, k[i]);
    }
    let ink = if state.set_note == DONE { r.text } else { r.danger };
    fb.text_scaled(s.px(NOTE_X), s.px(NOTE_Y), state.set_note, ink.argb(), s.px(NOTE_GLYPHS));
}
