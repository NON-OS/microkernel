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

//! The 514 byte cell, encoded and decoded.

use super::geometry::{CELL_BYTES, HEADER_BYTES, PAYLOAD_BYTES};

/// A fixed length cell: a circuit id, a command and 509 bytes.
#[derive(Clone)]
pub struct Cell {
    pub circuit: u32,
    pub command: u8,
    pub payload: [u8; PAYLOAD_BYTES],
}

impl Cell {
    pub fn new(circuit: u32, command: u8) -> Self {
        Self { circuit, command, payload: [0u8; PAYLOAD_BYTES] }
    }

    /// The cell as it goes on the wire. Padding to the full width is not
    /// optional: a short cell is a framing error at the relay, not a small
    pub fn encode(&self) -> [u8; CELL_BYTES] {
        let mut out = [0u8; CELL_BYTES];
        out[..4].copy_from_slice(&self.circuit.to_be_bytes());
        out[4] = self.command;
        out[HEADER_BYTES..].copy_from_slice(&self.payload);
        out
    }

    /// Read one cell from the front of `bytes`, or `None` if fewer than a
    /// whole cell has arrived.
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < CELL_BYTES {
            return None;
        }
        let mut payload = [0u8; PAYLOAD_BYTES];
        payload.copy_from_slice(&bytes[HEADER_BYTES..CELL_BYTES]);
        Some(Self {
            circuit: u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            command: bytes[4],
            payload,
        })
    }
}
