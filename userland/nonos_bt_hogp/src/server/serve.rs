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
//! Answering a request from the peripheral (Vol 3, Part F, 3.4) from the
//! fixed database. Each request gets its response or an Error Response
//! (3.4.1.1). A request this server does not take, such as Read Blob or
//! Prepare Write, gets Request Not Supported. No value is longer than a
//! Read Response holds, so none needs Read Blob.

use alloc::vec::Vec;

use super::find::{find_by_type_value, find_information};
use super::read::{read_by_group_type, read_by_type};
use super::single::{exchange_mtu, read, write};
use nonos_bt_wire::att::{self, error::*, opcode::*};
use nonos_bt_wire::cursor::Reader;

/// The ATT_MTU on LE until an exchange changes it (Vol 3, Part G, 5.2.1).
/// This server's responses never exceed it, which any ATT_MTU allows.
pub const ATT_MTU: usize = 23;
/// The largest ATT PDU this host receives, as client and as server: a
/// 251-octet LE data PDU less the 4-octet L2CAP header, and well within
/// the 512 octets the capsule reassembles.
pub const RX_MTU: u16 = 247;

/// A response, or the handle and code of an Error Response.
pub type Answer = Result<Vec<u8>, (u16, u8)>;

/// The response to request `op` with parameters `p`.
pub fn serve(op: u8, p: &[u8]) -> Vec<u8> {
    let answer = match op {
        EXCHANGE_MTU_REQ => exchange_mtu(p),
        FIND_INFORMATION_REQ => find_information(p),
        FIND_BY_TYPE_VALUE_REQ => find_by_type_value(p),
        READ_BY_TYPE_REQ => read_by_type(p),
        READ_REQ => read(p),
        READ_BY_GROUP_TYPE_REQ => read_by_group_type(p),
        WRITE_REQ => write(p),
        _ => Err((0, REQUEST_NOT_SUPPORTED)),
    };
    answer.unwrap_or_else(|(handle, code)| att::error_rsp(op, handle, code))
}

/// A malformed request: Invalid PDU (3.4.1.1).
pub const BAD: (u16, u8) = (0, INVALID_PDU);

/// The handle range that opens Find Information, Find By Type Value, Read
/// By Type and Read By Group Type. A start of 0 or past the end is an
/// Invalid Handle on the start (3.4.3.1, 3.4.3.3, 3.4.4.1, 3.4.4.9).
pub fn range(r: &mut Reader) -> Result<(u16, u16), (u16, u8)> {
    let (start, end) = (r.u16().map_err(|_| BAD)?, r.u16().map_err(|_| BAD)?);
    if start == 0 || start > end {
        return Err((start, INVALID_HANDLE));
    }
    Ok((start, end))
}
