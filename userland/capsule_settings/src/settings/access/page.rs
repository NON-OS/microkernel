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

//! A key on one of the page's rows. A new display is read from the
//! compositor; any other change is sent at once, and taken back when the
//! shell or the compositor refuses it, so the row never says what is not so.

use crate::settings::schema::access_knob::Knob;
use crate::settings::state::status::StatusKind;
use crate::settings::state::State;

use super::knobs::step;
use super::shell::{call, OP_ACCESSIBILITY, OP_ACCESSIBILITY_GET};
use super::wire::{ask, body, knobs};

pub fn adjust(state: &mut State, knob: Knob, delta: i32) {
    let before = state.access;
    if !step(&mut state.access, knob, delta) {
        return;
    }
    if knob == Knob::Output {
        return read(state);
    }
    if !before.read {
        state.access = before;
        return state.status.set(StatusKind::Error, b"not read from the display yet");
    }
    let b = body(state.access.output, &state.access.knobs);
    match call(OP_ACCESSIBILITY, &b, &mut []) {
        Ok(()) => state.status.set(StatusKind::Ok, b"applied to the display"),
        Err(_) => {
            state.access = before;
            state.status.set(StatusKind::Error, b"the display did not take it; nothing changed");
        }
    }
}

/// Enter or a click on the control: the next value, back to the first after
/// the last, as a switch or a list of choices goes round.
pub fn cycle(state: &mut State, knob: Knob) {
    let mut probe = state.access;
    let delta = if step(&mut probe, knob, 1) { 1 } else { -64 };
    adjust(state, knob, delta);
}

/// What the compositor holds for the page's display now.
pub fn read(state: &mut State) {
    let mut out = [0u8; 16];
    let got =
        ask(state.access.output).ok_or(-22).and_then(|b| call(OP_ACCESSIBILITY_GET, &b, &mut out));
    match got.ok().and_then(|()| knobs(&out)) {
        Some(k) => (state.access.knobs, state.access.read) = (k, true),
        None => {
            state.access.read = false;
            state.status.set(StatusKind::Error, b"the display's settings could not be read");
        }
    }
}
