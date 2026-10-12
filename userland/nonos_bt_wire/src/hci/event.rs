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

//! An event packet (Vol 4, Part E, 5.4.4): event code, a one-octet
//! parameter total length, then the parameters; and the two events every
//! command is answered with.

use super::opcode::Opcode;
use crate::cursor::Reader;
use crate::error::WireError;

pub struct Event<'a> {
    pub code: u8,
    pub params: &'a [u8],
}

pub fn parse_event(buf: &[u8]) -> Result<Event<'_>, WireError> {
    let mut r = Reader::new(buf);
    let code = r.u8()?;
    let len = usize::from(r.u8()?);
    let params = r.bytes(len)?;
    r.finish()?;
    Ok(Event { code, params })
}

/// Command Complete (Vol 4, Part E, 7.7.14): how many commands the
/// controller now accepts, the opcode answered, and its return parameters.
pub struct CommandComplete<'a> {
    pub num_packets: u8,
    pub opcode: Opcode,
    pub ret: &'a [u8],
}

/// Command Status (Vol 4, Part E, 7.7.15).
pub struct CommandStatus {
    pub status: u8,
    pub num_packets: u8,
    pub opcode: Opcode,
}

impl<'a> CommandComplete<'a> {
    pub fn parse(params: &'a [u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(params);
        let (num_packets, opcode) = (r.u8()?, Opcode(r.u16()?));
        Ok(Self { num_packets, opcode, ret: r.rest() })
    }
}

impl CommandStatus {
    pub fn parse(params: &[u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(params);
        let (status, num_packets, opcode) = (r.u8()?, r.u8()?, Opcode(r.u16()?));
        r.finish()?;
        Ok(Self { status, num_packets, opcode })
    }
}
