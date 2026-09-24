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

//! Variable length cells, and the circuit id width that varies with them.

extern crate alloc;

use alloc::vec::Vec;

use super::commands::CELL_VERSIONS;

/// A variable length cell: VERSIONS, VPADDING, CERTS, AUTH_CHALLENGE,
/// AUTHENTICATE.
pub struct VarCell {
    pub circuit: u32,
    pub command: u8,
    pub body: Vec<u8>,
}

/*
 * The circuit id is two bytes on a VERSIONS cell and four on every other
 * variable length cell. That is not a version negotiation the two ends can
 * disagree about: a VERSIONS cell is what establishes the link version, so
 * it has to be readable before one is known, and it is the one cell whose
 * width is fixed at two by the protocol regardless.
 */
fn circuit_width(command: u8) -> usize {
    if command == CELL_VERSIONS {
        2
    } else {
        4
    }
}

impl VarCell {
    /// Build one for sending.
    pub fn new(command: u8, body: Vec<u8>) -> Self {
        Self { circuit: 0, command, body }
    }

    pub fn encode(&self) -> Vec<u8> {
        let width = circuit_width(self.command);
        let mut out = Vec::with_capacity(width + 3 + self.body.len());
        let circuit = self.circuit.to_be_bytes();
        out.extend_from_slice(&circuit[4 - width..]);
        out.push(self.command);
        out.extend_from_slice(&(self.body.len() as u16).to_be_bytes());
        out.extend_from_slice(&self.body);
        out
    }
}
