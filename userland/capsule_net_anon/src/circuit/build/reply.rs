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

//! Reading a handshake reply off the link.

use crate::cell::{
    body, unpack, Cell, CELL_CREATED2, CELL_DESTROY, CELL_RELAY, CELL_RELAY_EARLY, RELAY_EXTENDED2,
    RELAY_TRUNCATED,
};
use crate::link::Link;

use super::error::BuildError;
use super::next_cell::next;

/// A CREATED2 payload for the guard, cleartext because no hop exists yet.
pub(super) fn created2(link: &mut Link, circuit: u32) -> Result<Cell, BuildError> {
    match next(link, circuit)? {
        cell if cell.command == CELL_CREATED2 => Ok(cell),
        cell if cell.command == CELL_DESTROY => Err(BuildError::Destroyed),
        _ => Err(BuildError::Protocol),
    }
}

pub(super) fn extended2(link: &mut Link, circuit: u32) -> Result<Cell, BuildError> {
    match next(link, circuit)? {
        cell if cell.command == CELL_RELAY || cell.command == CELL_RELAY_EARLY => Ok(cell),
        cell if cell.command == CELL_DESTROY => Err(BuildError::Destroyed),
        _ => Err(BuildError::Protocol),
    }
}

pub(super) fn extended2_body(
    payload: &[u8; crate::cell::PAYLOAD_BYTES],
) -> Result<&[u8], BuildError> {
    let header = unpack(payload);
    if header.command == RELAY_TRUNCATED {
        return Err(BuildError::Destroyed);
    }
    if header.command != RELAY_EXTENDED2 || header.stream != 0 {
        return Err(BuildError::Protocol);
    }
    body(payload).ok_or(BuildError::Protocol)
}
