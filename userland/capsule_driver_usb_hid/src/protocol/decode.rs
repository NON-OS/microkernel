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

use super::header::{Request, HDR_LEN, MAGIC, VERSION};

pub fn parse(buf: &[u8]) -> Option<(Request, &[u8])> {
    if buf.len() < HDR_LEN {
        return None;
    }
    let magic = u32::from_le_bytes(buf[0..4].try_into().ok()?);
    let version = u16::from_le_bytes(buf[4..6].try_into().ok()?);
    if magic != MAGIC || version != VERSION {
        return None;
    }
    let op = u16::from_le_bytes(buf[6..8].try_into().ok()?);
    let flags = u16::from_le_bytes(buf[8..10].try_into().ok()?);
    let request_id = u32::from_le_bytes(buf[12..16].try_into().ok()?);
    let payload_len = u32::from_le_bytes(buf[16..20].try_into().ok()?);
    let end = HDR_LEN.checked_add(payload_len as usize)?;
    if end != buf.len() {
        return None;
    }
    Some((Request { op, flags, request_id }, &buf[HDR_LEN..end]))
}

/// The request a frame `parse` refused is answered under: the op, flags and
/// request id it names, or zeros when it is too short to name them. Its
/// caller is blocked in its call until a reply comes, so a refusal is
/// answered too.
pub fn refused(buf: &[u8]) -> Request {
    let Some(h) = buf.first_chunk::<HDR_LEN>() else {
        return Request { op: 0, flags: 0, request_id: 0 };
    };
    Request {
        op: u16::from_le_bytes([h[6], h[7]]),
        flags: u16::from_le_bytes([h[8], h[9]]),
        request_id: u32::from_le_bytes([h[12], h[13], h[14], h[15]]),
    }
}
