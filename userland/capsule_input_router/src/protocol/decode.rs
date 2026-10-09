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

use super::{Request, E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION, HDR_LEN, MAGIC, VERSION};

/// The request a frame carries and its body, or why it was refused and the
/// request the refusal answers.
///
/// A refused frame is still answered: its caller is blocked on the reply and
/// would otherwise wait out its whole timeout. The refusal carries the op,
/// flags and request id the frame named, or zeros when it is too short to
/// name them, so a caller matching replies to calls can tell it is its own.
pub fn parse(buf: &[u8]) -> Result<(Request, &[u8]), (i32, Request)> {
    let Some(head) = buf.first_chunk::<HDR_LEN>() else {
        return Err((E_BAD_LEN, Request { op: 0, flags: 0, request_id: 0 }));
    };
    let req = Request { op: u16_at(head, 6), flags: u16_at(head, 8), request_id: u32_at(head, 12) };
    if u32_at(head, 0) != MAGIC {
        return Err((E_BAD_MAGIC, req));
    }
    if u16_at(head, 4) != VERSION {
        return Err((E_BAD_VERSION, req));
    }
    match HDR_LEN.checked_add(u32_at(head, 16) as usize) {
        Some(end) if end == buf.len() => Ok((req, &buf[HDR_LEN..])),
        _ => Err((E_BAD_LEN, req)),
    }
}

fn u16_at(head: &[u8; HDR_LEN], off: usize) -> u16 {
    u16::from_le_bytes([head[off], head[off + 1]])
}

fn u32_at(head: &[u8; HDR_LEN], off: usize) -> u32 {
    u32::from_le_bytes([head[off], head[off + 1], head[off + 2], head[off + 3]])
}
