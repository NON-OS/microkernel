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

//! Delivered payload, and the windows it spends.

use crate::circuit::window::circuit_sendme_due;

use super::super::state::Manager;

/*
 * Circuit level flow control lives on the last hop, not on the circuit, because
 * that is the hop the windows are negotiated with and the hop a SENDME is
 * addressed to. Keeping a second copy on the circuit would be two truths about
 * one window.
 */
pub(super) fn data(state: &mut Manager, index: usize, id: u16, body: &[u8]) {
    let circuit = &mut state.circuits[index];
    if let Some(target) = circuit.hops.len().checked_sub(1) {
        let hop = &mut circuit.hops[target];
        hop.deliver_window = hop.deliver_window.saturating_sub(1);
    }
    circuit.delivered_since = circuit.delivered_since.saturating_add(1);
    /*
     * The digest is pinned here, at the cell that brought the count to the
     * increment, and not read again when the SENDME is actually sent. The far end
     * checks the digest against the cell it recorded at that same position, so
     * acknowledging with whatever arrived most recently is a digest it never
     * recorded, and a SENDME it does not recognise closes the circuit.
     */
    if circuit_sendme_due(circuit.delivered_since) && circuit.owed_digest.is_none() {
        if let Some(target) = circuit.hops.len().checked_sub(1) {
            circuit.owed_digest = Some(circuit.hops[target].last_seen);
        }
    }
    /*
     * Payload, not acknowledgement, is what proves an exit carries traffic. An
     * exit that answers every control message and delivers nothing is the failure
     * the mixnet transport lost a session to, so the mark is set here and nowhere
     * else, and only for a body with something in it.
     */
    if !body.is_empty() {
        circuit.proven = true;
    }
    let Some(stream) = state.streams.iter_mut().find(|s| s.id == id) else {
        return;
    };
    stream.deliver_window = stream.deliver_window.saturating_sub(1);
    stream.delivered_since = stream.delivered_since.saturating_add(1);
    stream.inbound.extend_from_slice(body);
}
