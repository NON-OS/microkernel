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

//! Parse a net_core request header, rejecting anything that is not a well-
//! formed message for this version.

use super::*;

/// Parse a net_core request header, rejecting anything that is not a well-formed
/// message for this version.
pub fn parse_request(buf: &[u8]) -> Option<Request<'_>> {
    if buf.len() < HDR_LEN {
        return None;
    }
    if u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) != MAGIC_NNET {
        return None;
    }
    if u16::from_le_bytes([buf[4], buf[5]]) != VERSION {
        return None;
    }
    let op = u16::from_le_bytes([buf[6], buf[7]]);
    let request_id = u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]);
    let payload_len = u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]) as usize;
    let end = HDR_LEN.checked_add(payload_len)?;
    if end > buf.len() {
        return None;
    }
    Some(Request { op, request_id, payload: &buf[HDR_LEN..end] })
}

/// Write a response header into `out`, returning the header length or None if
/// the buffer is too small.
pub fn write_response_header(
    out: &mut [u8],
    op: u16,
    request_id: u32,
    payload_len: u32,
) -> Option<usize> {
    if out.len() < HDR_LEN {
        return None;
    }
    out[0..4].copy_from_slice(&MAGIC_NNET.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..10].copy_from_slice(&0u16.to_le_bytes());
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&payload_len.to_le_bytes());
    Some(HDR_LEN)
}
