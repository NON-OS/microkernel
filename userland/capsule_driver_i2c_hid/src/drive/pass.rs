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

//! One pass of the serving loop, between two waits on its inbox: whatever
//! is due of looking for the pad, looking at its line and reading it.

use super::doorbell::{look, Look};
use super::rang::after_ring;
use super::reset::set_up_owed;
use crate::input::poll;
use crate::setup::reprobe;
use crate::state::State;

/// While no pad is bound the controller is asked for one this often, and
/// a pad that answers late (the controller still bringing its bus up, the
/// pad still powering on) is found on a later ask.
pub const REPROBE_MS: u64 = 500;

pub fn pass(state: &mut State, now_ms: u64) {
    if state.done || !state.pace.due(now_ms) {
        return;
    }
    if !state.found() {
        reprobe(state);
        state.pace.after(now_ms, if state.found() { 0 } else { REPROBE_MS });
        return;
    }
    // A set-up a reset is owed waits for the holdoff, not for a report.
    set_up_owed(state);
    let rang = match look(state, now_ms) {
        Look::Quiet => {
            state.pace.read(now_ms, false);
            return;
        }
        Look::Rang => true,
        Look::Timed => false,
    };
    let fresh = poll(state);
    if rang {
        after_ring(state);
    }
    state.pace.read(now_ms, fresh);
}
