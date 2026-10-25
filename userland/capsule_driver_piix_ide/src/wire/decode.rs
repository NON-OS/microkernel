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

/// The header of `buf`, or None for a frame that is short or not the wire.
pub fn decode_request(buf: &[u8]) -> Option<Request> {
    if buf.len() < HDR_LEN {
        return None;
    }
    let u16_at = |at: usize| u16::from_le_bytes([buf[at], buf[at + 1]]);
    let u32_at = |at: usize| u32::from_le_bytes([buf[at], buf[at + 1], buf[at + 2], buf[at + 3]]);
    if u32_at(0) != MAGIC || u16_at(4) != VERSION {
        return None;
    }
    Some(Request {
        op: u16_at(6),
        flags: u16_at(8),
        unit: u16_at(10),
        request_id: u32_at(12),
        payload_len: u32_at(16),
    })
}
