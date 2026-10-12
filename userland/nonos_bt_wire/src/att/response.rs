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

//! The PDUs a client receives (Vol 3, Part F, 3.4). Each parser takes the
//! PDU's parameters, after the opcode.

use crate::cursor::Reader;
use crate::error::WireError;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ErrorRsp {
    pub request: u8,
    pub handle: u16,
    pub code: u8,
}

/// Error Response (3.4.1.1).
pub fn error_rsp(p: &[u8]) -> Result<ErrorRsp, WireError> {
    let mut r = Reader::new(p);
    let e = ErrorRsp { request: r.u8()?, handle: r.u16()?, code: r.u8()? };
    r.finish()?;
    Ok(e)
}

/// Find By Type Value Response (3.4.3.4): handle ranges, at least one.
pub fn found_ranges(p: &[u8]) -> Result<Vec<(u16, u16)>, WireError> {
    if p.is_empty() || !p.len().is_multiple_of(4) {
        return Err(WireError::Length);
    }
    let range = |c: &[u8]| (u16::from_le_bytes([c[0], c[1]]), u16::from_le_bytes([c[2], c[3]]));
    Ok(p.chunks_exact(4).map(range).collect())
}

/// Read By Type Response (3.4.4.2) and Find Information Response
/// (3.4.3.2) share a shape: a handle then a fixed-length value, repeated.
/// `each` is that pair's length, from the PDU's own length or format
/// field.
pub fn handle_pairs(list: &[u8], each: usize) -> Result<Vec<(u16, &[u8])>, WireError> {
    if each < 2 || list.is_empty() || !list.len().is_multiple_of(each) {
        return Err(WireError::Length);
    }
    Ok(list.chunks_exact(each).map(|c| (u16::from_le_bytes([c[0], c[1]]), &c[2..])).collect())
}

/// A notification or indication (3.4.7.1, 3.4.7.2): the handle and value.
#[derive(Debug, PartialEq, Eq)]
pub struct HandleValue<'a> {
    pub handle: u16,
    pub value: &'a [u8],
}

pub fn handle_value(p: &[u8]) -> Result<HandleValue<'_>, WireError> {
    let mut r = Reader::new(p);
    let handle = r.u16()?;
    Ok(HandleValue { handle, value: r.rest() })
}
