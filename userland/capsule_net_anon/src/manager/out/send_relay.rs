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

//! Sealing one relay message and putting it on the link.

use crate::cell::{pack, Cell, RelayHeader, CELL_RELAY};
use crate::circuit::seal;

use super::super::state::Manager;

/// Why an outbound message could not be sent.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SendError {
    NoLink,
    NoCircuit,
    NoStream,
    TableFull,
    Ended(u8),
    WouldBlock,
    TooLong,
    Link,
}

pub(crate) fn send_relay(
    state: &mut Manager,
    index: usize,
    command: u8,
    stream: u16,
    body: &[u8],
) -> Result<(), SendError> {
    let circuit = state.circuits.get_mut(index).ok_or(SendError::NoCircuit)?;
    let target = circuit.last_hop().ok_or(SendError::NoCircuit)?;
    let header = RelayHeader { command, recognized: 0, stream, length: body.len() as u16 };
    let mut payload = pack(&header, body).ok_or(SendError::TooLong)?;
    seal(&mut circuit.hops, target, &mut payload).ok_or(SendError::NoCircuit)?;
    let id = circuit.id;

    let mut cell = Cell::new(id, CELL_RELAY);
    cell.payload = payload;
    let link = state.link.as_mut().ok_or(SendError::NoLink)?;
    link.send(&cell.encode()).map_err(|_| SendError::Link)
}
