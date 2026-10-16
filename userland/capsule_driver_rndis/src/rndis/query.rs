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

//! REMOTE_NDIS_QUERY_MSG and the information buffer of its completion
//! (Remote NDIS 1.0, 2.2.4 and 2.2.5), built and bounded here.

use alloc::vec;
use alloc::vec::Vec;

use super::message::{le32, put32, MSG_QUERY};

/// The bytes of a QUERY with no information buffer, before its payload.
const QUERY_LEN: usize = 28;
/// A QUERY_CMPLT's fields before its information buffer.
const CMPLT_HDR: usize = 24;
/// Zero bytes sent after the query for the station address: ActiveSync
/// devices want a buffer as large as their answer (observed, unverified;
/// 48 bytes covers the address).
pub const ADDRESS_PAYLOAD: usize = 48;

/// A QUERY for `oid` with `payload` zero bytes after it. The offset is
/// counted from the RequestID field, as every RNDIS offset is.
pub fn query_msg(oid: u32, payload: usize) -> Vec<u8> {
    let mut m = vec![0u8; QUERY_LEN + payload];
    let len = m.len() as u32;
    for (at, v) in [(0, MSG_QUERY), (4, len), (12, oid), (16, payload as u32), (20, 20)] {
        put32(&mut m, at, v);
    }
    m
}

/// The information buffer of a QUERY_CMPLT of `r.len()` bytes (its
/// MessageLength), or `None` when its length or offset reach past it or a
/// buffer starts inside the completion's own 24-byte header.
pub fn info(r: &[u8]) -> Option<&[u8]> {
    let len = le32(r, 16)? as usize;
    let start = (le32(r, 20)? as usize).checked_add(8)?;
    if len != 0 && start < CMPLT_HDR {
        return None;
    }
    r.get(start..start.checked_add(len)?)
}

/// The permanent station address: exactly six bytes, an 802.3 address,
/// and one the stack can send from.
pub fn station(r: &[u8]) -> Option<[u8; 6]> {
    let b = info(r).filter(|b| b.len() == 6)?;
    let mac = [b[0], b[1], b[2], b[3], b[4], b[5]];
    nonos_usbnet::desc::usable(mac).then_some(mac)
}
