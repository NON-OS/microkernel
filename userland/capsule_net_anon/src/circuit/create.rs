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

//! The CREATE2 cell, its reply, and how a client numbers a circuit.

use crate::cell::{Cell, CELL_CREATE2, HANDSHAKE_NTOR};

/// A CREATE2 cell carrying an ntor onion skin to the guard.
pub fn create2(circuit: u32, onionskin: &[u8]) -> Option<Cell> {
    let mut cell = Cell::new(circuit, CELL_CREATE2);
    let total = 4 + onionskin.len();
    if total > cell.payload.len() {
        return None;
    }
    cell.payload[..2].copy_from_slice(&HANDSHAKE_NTOR.to_be_bytes());
    cell.payload[2..4].copy_from_slice(&(onionskin.len() as u16).to_be_bytes());
    cell.payload[4..total].copy_from_slice(onionskin);
    Some(cell)
}

/// The handshake reply inside a CREATED2 payload.
pub fn created2_reply(payload: &[u8]) -> Option<&[u8]> {
    if payload.len() < 2 {
        return None;
    }
    let length = u16::from_be_bytes([payload[0], payload[1]]) as usize;
    payload.get(2..2 + length)
}

/*
 * Circuit ids a client originates must have their high bit set at link
 * protocol 4 and later, which is how the two ends of a link avoid colliding
 * without negotiating a range. Zero is reserved for cells that belong to the
 * link rather than to any circuit, so it is excluded.
 */
pub fn client_circuit_id(counter: u32) -> u32 {
    0x8000_0000 | (counter & 0x7fff_ffff).max(1)
}
