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

//! Reporting the relays a circuit runs over.

use crate::circuit::CircuitStage;
use crate::manager::Manager;
use crate::protocol::{E_NO_CIRCUIT, E_OK, HDR_LEN};

const PER_HOP: usize = 6;

/*
 * The addresses, not the identities. A front end showing a route wants somewhere
 * a user can recognise, and handing out identity keys over IPC would put the
 * material a circuit is authenticated with in reach of every caller.
 */
/// Body: circuit id, hop count, then the hops. `E_NO_CIRCUIT` if it is not open.
pub fn circuit_path(state: &Manager, body: &[u8], tx: &mut [u8]) -> (u16, u32) {
    if body.len() < 4 {
        return (E_NO_CIRCUIT, 0);
    }
    let id = u32::from_le_bytes([body[0], body[1], body[2], body[3]]);
    let Some(circuit) = state.circuits.iter().find(|c| c.id == id) else {
        return (E_NO_CIRCUIT, 0);
    };
    if circuit.stage != CircuitStage::Open {
        return (E_NO_CIRCUIT, 0);
    }
    tx[HDR_LEN..HDR_LEN + 4].copy_from_slice(&id.to_le_bytes());
    tx[HDR_LEN + 4] = circuit.path.len() as u8;
    let mut at = HDR_LEN + 5;
    for relay in circuit.path.iter() {
        tx[at..at + 4].copy_from_slice(&relay.address);
        tx[at + 4..at + 6].copy_from_slice(&relay.or_port.to_le_bytes());
        at += PER_HOP;
    }
    (E_OK, (5 + circuit.path.len() * PER_HOP) as u32)
}
