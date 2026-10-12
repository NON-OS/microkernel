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

//! An ACL data packet (Vol 4, Part E, 5.4.2): the 12-bit connection handle
//! with the 2-bit Packet_Boundary flag and 2-bit Broadcast flag above it,
//! a 16-bit data total length, then the data. Handles above 0x0EFF are
//! reserved, and LE has no broadcast ACL, so its Broadcast flag is 0b00.

use alloc::vec::Vec;

use super::boundary::Boundary;
use crate::cursor::{Reader, Writer};
use crate::error::WireError;

pub const HANDLE_MAX: u16 = 0x0eff;

pub struct AclHeader {
    pub handle: u16,
    pub boundary: Boundary,
    pub len: u16,
}

pub fn parse_acl(buf: &[u8]) -> Result<(AclHeader, &[u8]), WireError> {
    let mut r = Reader::new(buf);
    let word = r.u16()?;
    let len = r.u16()?;
    let (handle, pb, bc) = (word & 0x0fff, (word >> 12) & 0b11, word >> 14);
    if handle > HANDLE_MAX || bc != 0 {
        return Err(WireError::Reserved);
    }
    let boundary = Boundary::from_bits(pb);
    let data = r.bytes(usize::from(len))?;
    r.finish()?;
    Ok((AclHeader { handle, boundary, len }, data))
}

pub fn acl(handle: u16, boundary: Boundary, data: &[u8]) -> Result<Vec<u8>, WireError> {
    if handle > HANDLE_MAX {
        return Err(WireError::Reserved);
    }
    let len = u16::try_from(data.len()).map_err(|_| WireError::TooLong)?;
    let pb = boundary.bits();
    let mut w = Writer::new();
    w.u16(handle | (pb << 12)).u16(len).bytes(data);
    Ok(w.into_vec())
}
