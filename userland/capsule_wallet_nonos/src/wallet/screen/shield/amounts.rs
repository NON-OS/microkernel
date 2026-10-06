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

/*
 * Shielded balances as the shield service wrote them, with what is still
 * maturing beside it, and a dash until the service has read the pool.
 */

use alloc::string::String;

use crate::wallet::screen::amounts::UNREAD;
use crate::wallet::state::State;

fn held(state: &State, slot: usize) -> String {
    match &state.shield_ui.held[slot] {
        Some(h) if h.pending != "0" && !h.pending.is_empty() => {
            alloc::format!("{} (+{} pending)", h.spendable, h.pending)
        }
        Some(h) => h.spendable.clone(),
        None => String::from(UNREAD),
    }
}

pub fn shielded_eth(state: &State) -> String {
    held(state, 0)
}

pub fn shielded_nox(state: &State) -> String {
    held(state, 1)
}
