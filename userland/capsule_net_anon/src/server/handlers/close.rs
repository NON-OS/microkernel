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

//! Closing a stream, and tearing down a circuit.

use crate::circuit::destroy;
use crate::manager::Manager;
use crate::protocol::{E_NO_CIRCUIT, E_NO_STREAM, E_OK};
use crate::stream::{StreamStage, REASON_DONE};

/// Close one stream, leaving its circuit up for the next one.
pub fn close_stream(state: &mut Manager, body: &[u8]) -> u16 {
    if body.len() < 2 {
        return E_NO_STREAM;
    }
    let id = u16::from_le_bytes([body[0], body[1]]);
    let Some(stream) = state.streams.iter().find(|s| s.id == id) else {
        return E_NO_STREAM;
    };
    let circuit_id = stream.circuit;
    if let Some(index) = state.circuits.iter().position(|c| c.id == circuit_id) {
        let _ = crate::manager::send_end(state, index, id, REASON_DONE);
    }
    state.streams.retain(|s| s.id != id);
    E_OK
}

/*
 * A separate operation from closing a stream, and deliberately so. In the mixnet
 * transport one call did both jobs, and the day a rotation forgot the second half
 * every later rebind failed for the life of the boot.
 */
/// Tear down one circuit and drop every stream on it.
pub fn close_circuit(state: &mut Manager, body: &[u8]) -> u16 {
    if body.len() < 4 {
        return E_NO_CIRCUIT;
    }
    let id = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);
    if !state.circuits.iter().any(|c| c.id == id) {
        return E_NO_CIRCUIT;
    }
    for stream in state.streams.iter_mut().filter(|s| s.circuit == id) {
        stream.stage = StreamStage::Ended(REASON_DONE);
    }
    state.streams.retain(|s| s.circuit != id);
    /*
     * DESTROY while the circuit is still the link's, as tor-spec 5.4 asks. Dropping
     * it silently leaves three relays holding it until their own timeouts, and a
     * client that never sends one is a client that looks like no other.
     */
    if let Some(link) = state.link.as_mut() {
        let _ = link.send(&destroy(id).encode());
    }
    state.circuits.retain(|c| c.id != id);
    E_OK
}
