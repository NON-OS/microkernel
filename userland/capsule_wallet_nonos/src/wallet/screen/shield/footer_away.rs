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

//! The footer's one press when the shield is on another network.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::state::State;

/* The one press off the shield's network: go to Sepolia and open it. */
pub(super) fn elsewhere(state: &mut State, n: u8) -> EventOutcome {
    if n != 0 {
        return EventOutcome::Idle;
    }
    match crate::wallet::state::switch_network(state, true) {
        Ok(_) => {
            state.status = b"network changed";
            super::click::open(state)
        }
        Err(why) => {
            state.failure = Some(why);
            EventOutcome::Repaint
        }
    }
}
