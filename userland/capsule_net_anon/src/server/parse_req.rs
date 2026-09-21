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

//! Reading a request header.

use crate::protocol::{E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION, HDR_LEN, MAGIC, VERSION};

pub struct Request {
    pub op: u16,
    pub request_id: u32,
}

/// Split a request into its header and body.
///
pub fn parse(buf: &[u8]) -> Result<(Request, &[u8]), u16> {
    if buf.len() < HDR_LEN {
        return Err(E_BAD_LEN);
    }
    if le32(buf, 0) != MAGIC {
        return Err(E_BAD_MAGIC);
    }
    if le16(buf, 4) != VERSION {
        return Err(E_BAD_VERSION);
    }
    let payload_len = le32(buf, 16) as usize;
    let want = HDR_LEN.checked_add(payload_len).ok_or(E_BAD_LEN)?;
    if buf.len() < want {
        return Err(E_BAD_LEN);
    }
    Ok((Request { op: le16(buf, 6), request_id: le32(buf, 12) }, &buf[HDR_LEN..want]))
}

fn le16(buf: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([buf[off], buf[off + 1]])
}

fn le32(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}
