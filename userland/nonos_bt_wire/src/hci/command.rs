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

//! A command packet (Vol 4, Part E, 5.4.1): the opcode, a one-octet
//! parameter total length, then the parameters. Over USB it goes alone in a
//! control transfer with no packet indicator (Vol 4, Part B, 2); over UART
//! the indicator 0x01 comes first (Vol 4, Part A, 2), which is the
//! transport's to add, not this crate's.

use alloc::vec::Vec;

use super::opcode::Opcode;
use crate::cursor::{Reader, Writer};
use crate::error::WireError;

/// The command; `TooLong` if the parameters exceed the 255 octets one
/// length octet can state.
pub fn command(op: Opcode, params: &[u8]) -> Result<Vec<u8>, WireError> {
    let len = u8::try_from(params.len()).map_err(|_| WireError::TooLong)?;
    let mut w = Writer::new();
    w.u16(op.0).u8(len).bytes(params);
    Ok(w.into_vec())
}

/// The opcode and parameters of a command packet, as a controller (or a
/// simulated one in the proofs) reads it.
pub fn parse_command(buf: &[u8]) -> Result<(Opcode, &[u8]), WireError> {
    let mut r = Reader::new(buf);
    let op = Opcode(r.u16()?);
    let len = usize::from(r.u8()?);
    let params = r.bytes(len)?;
    r.finish()?;
    Ok((op, params))
}
