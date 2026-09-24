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

//! Paying the SENDMEs a circuit owes.

use crate::cell::RELAY_SENDME;
use crate::circuit::window::CIRCUIT_INCREMENT;
use crate::protocol::CIRCUIT_HIGH_WATER;
use crate::stream::sendme_body;

use super::super::state::Manager;
use super::buffered::buffered;
use super::send_relay::send_relay;
use super::stream_sendme::stream_tick;

/*
 * Two windows, two acknowledgements, and both are required. The circuit window
 * closes after a hundred cells and the stream window after fifty, so paying only
 * the circuit one stalls every stream at fifty cells with the circuit still wide
 * open, which looks like a site that stops loading half way.
 */
pub fn tick(state: &mut Manager) {
    circuit_tick(state);
    stream_tick(state);
}

fn circuit_tick(state: &mut Manager) {
    for index in 0..state.circuits.len() {
        let Some(digest) = state.circuits[index].owed_digest else { continue };
        let Some(target) = state.circuits[index].last_hop() else { continue };
        // Held back on the total buffered, not per stream: see `buffered`.
        if buffered(state, state.circuits[index].id) > CIRCUIT_HIGH_WATER {
            continue;
        }
        if send_relay(state, index, RELAY_SENDME, 0, &sendme_body(&digest)).is_err() {
            continue;
        }
        let circuit = &mut state.circuits[index];
        circuit.owed_digest = None;
        circuit.delivered_since -= CIRCUIT_INCREMENT;
        circuit.hops[target].deliver_window =
            circuit.hops[target].deliver_window.saturating_add(CIRCUIT_INCREMENT);
    }
}
