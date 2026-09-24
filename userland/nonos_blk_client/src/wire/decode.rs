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

//! Reply validation. `n` is the byte count the kernel reported, clamped
//! against the buffer so a bogus length can never widen the slice past
//! what was received. The payload after the status word is handed back.

use super::{HDR_LEN, STATUS_LEN, VERSION};
use crate::error::BlkError;

pub fn decode_reply(
    rx: &[u8],
    n: usize,
    magic: u32,
    op: u16,
    request_id: u32,
) -> Result<&[u8], BlkError> {
    if n < HDR_LEN + STATUS_LEN || n > rx.len() {
        return Err(BlkError::ShortReply(n));
    }
    if le32(rx, 0) != magic || le16(rx, 4) != VERSION || le16(rx, 6) != op {
        return Err(BlkError::BadHeader);
    }
    if le32(rx, 12) != request_id {
        return Err(BlkError::IdMismatch);
    }
    let payload_len = le32(rx, 16) as usize;
    if payload_len < STATUS_LEN || HDR_LEN + payload_len > n {
        return Err(BlkError::BadLength);
    }
    let status = le32(rx, HDR_LEN) as i32;
    if status != 0 {
        return Err(BlkError::Status(status));
    }
    Ok(&rx[HDR_LEN + STATUS_LEN..HDR_LEN + payload_len])
}

fn le16(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}

fn le32(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}
