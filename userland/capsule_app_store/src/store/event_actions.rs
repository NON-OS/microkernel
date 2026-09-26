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

//! The keys that do something rather than move somewhere.

use nonos_app_skeleton::{EventOutcome, KEY_ENTER};

use super::install;
use super::state::State;

const KEY_O: u32 = b'o' as u32;
const KEY_R: u32 = b'r' as u32;

pub(super) fn act(state: &mut State, code: u32) -> EventOutcome {
    let changed = match code {
        KEY_ENTER => {
            state.asked = Some(install::ask(state));
            true
        }
        /*
         * Whether the program may start is not this window's answer: the
         * kernel queues the run, and the exec gate checks the trailer the
         * machine minted at install under the consent given in setup.
         */
        KEY_O => {
            state.asked = Some(install::open(state));
            true
        }
        KEY_R => {
            state.asked = None;
            state.refresh();
            true
        }
        _ => false,
    };
    match changed {
        true => EventOutcome::Repaint,
        false => EventOutcome::Idle,
    }
}
