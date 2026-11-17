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
use nonos_toolkit::theme::contrast::{at_least, BODY};
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{Button, Kind};

use crate::clock::controls::{keys, Target};
use crate::clock::fmt;
use crate::clock::hover::look;
use crate::clock::state::State;
use crate::clock::tabs::Tab;

/// The wash a fired timer lays under the tabs, and where the readout sits, in
/// logical pixels; the readout's glyph size is a multiple of the 8 pixel
/// bitmap face at 1x.
const WASH_TOP: u32 = 60;
const WASH_W: u32 = 360;
const WASH_H: u32 = 380;
const READOUT_X: u32 = 84;
const READOUT_Y: u32 = 150;
const READOUT_GLYPHS: u32 = 6;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let (r, s) = (roles(), scale());
    let (danger, ground) = (r.danger.argb(), r.bg.argb());
    let mut ink = r.text.argb();
    if state.timer.fired {
        fb.fill_rect(0, s.px(WASH_TOP), s.px(WASH_W), s.px(WASH_H), danger);
        ink = if at_least(ground, danger, BODY) { ground } else { ink };
    }
    let rem = state.timer.remaining(state.now_ms);
    let buf = fmt::ms_ms(rem);
    fb.text_scaled(s.px(READOUT_X), s.px(READOUT_Y), &buf, ink, s.px(READOUT_GLYPHS));
    let label = if state.timer.running { "Pause" } else { "Start" };
    let k = keys(Tab::Timer);
    for (i, name) in ["1m", "5m", "10m", label, "Reset"].into_iter().enumerate() {
        let kind = if i == 3 { Kind::Primary } else { Kind::Secondary };
        Button::new(name).kind(kind).state(look(state, Target::Key(i))).draw(fb, k[i]);
    }
}
