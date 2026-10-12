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

//! The PDUs a client sends (Vol 3, Part F, 3.4), and the Error Response a
//! client with no attribute database gives every request it receives.

use super::opcode::*;
use crate::cursor::Writer;
use alloc::vec::Vec;

/// Find By Type Value Request for a 16-bit type and value (3.4.3.3).
pub fn find_by_type_value(start: u16, end: u16, kind: u16, value: &[u8]) -> Vec<u8> {
    let mut w = Writer::new();
    w.u8(FIND_BY_TYPE_VALUE_REQ).u16(start).u16(end).u16(kind).bytes(value);
    w.into_vec()
}

/// Read By Type Request for a 16-bit type (3.4.4.1).
pub fn read_by_type(start: u16, end: u16, kind: u16) -> Vec<u8> {
    let mut w = Writer::new();
    w.u8(READ_BY_TYPE_REQ).u16(start).u16(end).u16(kind);
    w.into_vec()
}

/// Find Information Request (3.4.3.1).
pub fn find_information(start: u16, end: u16) -> Vec<u8> {
    let mut w = Writer::new();
    w.u8(FIND_INFORMATION_REQ).u16(start).u16(end);
    w.into_vec()
}

/// Write Request (3.4.5.1), or Write Command (3.4.5.3) if `command`.
pub fn write(handle: u16, value: &[u8], command: bool) -> Vec<u8> {
    let mut w = Writer::new();
    w.u8(if command { WRITE_CMD } else { WRITE_REQ }).u16(handle).bytes(value);
    w.into_vec()
}

/// Handle Value Confirmation (3.4.7.3).
pub fn confirmation() -> Vec<u8> {
    alloc::vec![HANDLE_VALUE_CFM]
}

/// Error Response to `request` (3.4.1.1).
pub fn error_rsp(request: u8, handle: u16, code: u8) -> Vec<u8> {
    let mut w = Writer::new();
    w.u8(ERROR_RSP).u8(request).u16(handle).u8(code);
    w.into_vec()
}
