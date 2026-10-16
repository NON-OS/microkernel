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

//! A key on one of the displays' rows. Choosing which display the scale is
//! for is the page's own; a new arrangement or scale is sent at once, then
//! read back, so the rows say what the screens do, and is taken back when
//! the shell or the compositor refuses it.

use crate::settings::access::shell::{call, OP_DISPLAY_LAYOUT, OP_DISPLAY_SCALE};
use crate::settings::schema::access_knob::Knob;
use crate::settings::state::status::StatusKind;
use crate::settings::state::State;

use super::arrange::step;
use super::read::read;
use super::wire::{layout, scale};

pub fn adjust(state: &mut State, knob: Knob, delta: i32) {
    let before = state.screens;
    if !before.read {
        return state.status.set(StatusKind::Error, b"not read from the display yet");
    }
    if !step(&mut state.screens, knob, delta) || knob == Knob::Screen {
        return;
    }
    let p = state.screens;
    let sent = match knob {
        Knob::Arrange => call(OP_DISPLAY_LAYOUT, &layout(p.shown.extend), &mut []),
        _ => scale(p.output, p.shown.chosen[p.output as usize])
            .ok_or(-22)
            .and_then(|b| call(OP_DISPLAY_SCALE, &b, &mut [])),
    };
    match sent {
        Ok(()) => {
            // Read after: a read that fails says so over the success.
            state.status.set(StatusKind::Ok, b"applied to the displays");
            read(state);
        }
        Err(_) => {
            state.screens = before;
            state.status.set(StatusKind::Error, b"the displays did not take it; nothing changed");
        }
    }
}

/// Enter or a click: the next value, back to the first after the last.
pub fn cycle(state: &mut State, knob: Knob) {
    let mut probe = state.screens;
    let delta = if step(&mut probe, knob, 1) { 1 } else { -64 };
    adjust(state, knob, delta);
}
