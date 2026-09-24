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

//! Charging a stream failure to the exit that caused it.

use crate::protocol::CIRCUIT_FAILURES_MAX;
use crate::trace;

use super::super::state::Manager;

/*
 * Charged only for the reasons that will not improve however often the same exit
 * is asked, and only while the circuit has never delivered payload. On a circuit
 * that has, the site is the likely cause and retiring a working path over it
 * costs three handshakes for nothing; on one that has not, an exit acking control
 * messages and refusing every request is the shape the mixnet transport lost a
 * session to.
 */
pub(super) fn blame(state: &mut Manager, circuit: Option<u32>) {
    let Some(id) = circuit else { return };
    let Some(c) = state.circuits.iter_mut().find(|c| c.id == id) else {
        return;
    };
    if c.proven {
        return;
    }
    c.failures = c.failures.saturating_add(1);
    if c.failures >= CIRCUIT_FAILURES_MAX {
        trace::say_num(b"exit failed twice, circuit retiring", id as u64);
    }
}
