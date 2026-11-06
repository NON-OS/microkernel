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

use nonos_app_skeleton::EventOutcome;

use super::hover::rehover;
use super::on_convert;
use super::on_history;
use crate::calc::actions::{dispatch, prog_bit};
use crate::calc::buttons::grid;
use crate::calc::hit::Hit;
use crate::calc::state::State;
use crate::calc::ui::target::at;

fn outcome(changed: bool) -> EventOutcome {
    if changed {
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}

/// A click does what it did before. The press is held on the control it began
/// on until the release, and the control under the pointer is named again,
/// since the click may have changed the mode.
pub fn on_pointer_button(state: &mut State, x: i32, y: i32) -> EventOutcome {
    state.pointer.pressed(x, y);
    let out = click(state, x, y);
    rehover(state);
    if out == EventOutcome::Idle && state.hover.is_none() {
        return EventOutcome::Idle;
    }
    EventOutcome::Repaint
}

fn click(state: &mut State, x: i32, y: i32) -> EventOutcome {
    let hit = match at(state, x, y) {
        Some(found) => found,
        None => return EventOutcome::Idle,
    };
    match hit {
        Hit::Rail(mode) => state.set_mode(mode),
        Hit::Convert(control) => on_convert::click(state, control),
        Hit::Row(row) => return outcome(on_history::click(state, row)),
        Hit::Bit(bit) => prog_bit::run(state, bit),
        Hit::Key(row, idx) => {
            let button = match grid(state.mode).get(row).and_then(|r| r.get(idx)) {
                Some(btn) => *btn,
                None => return EventOutcome::Idle,
            };
            dispatch::run(state, button.action);
        }
    }
    EventOutcome::Repaint
}
