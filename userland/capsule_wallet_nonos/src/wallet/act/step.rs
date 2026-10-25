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
//! One slice of the running press's work, and its result applied once it ends.

use crate::wallet::state::State;

use super::Action;

/// One step of the running action; true when its result changed the screen.
pub fn step(state: &mut State) -> bool {
    let finished = match state.action.as_mut() {
        Some(Action::Review { exchange, .. }) | Some(Action::Broadcast { exchange, .. }) => {
            exchange.step()
        }
        None => return false,
    };
    let Some(ended) = finished else {
        return false;
    };
    match state.action.take() {
        Some(Action::Review { plan, .. }) => super::review::finish(state, plan, ended),
        Some(Action::Broadcast { purpose, hash, chain, address, .. }) => {
            super::broadcast::finish(state, purpose, hash, chain, address, ended)
        }
        None => {}
    }
    true
}
