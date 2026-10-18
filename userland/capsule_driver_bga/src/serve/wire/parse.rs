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

//! A request's frame, held to the wire before anything in it is believed.

use super::header::{Request, HDR_LEN, MAGIC, VERSION};

/// The request and its body, or `None` for a frame shorter than the header,
/// of another magic or version, or whose body is not the length it names.
pub fn parse(buf: &[u8]) -> Option<(Request, &[u8])> {
    let h = buf.first_chunk::<HDR_LEN>()?;
    let magic = u32::from_le_bytes([h[0], h[1], h[2], h[3]]);
    let version = u16::from_le_bytes([h[4], h[5]]);
    if magic != MAGIC || version != VERSION {
        return None;
    }
    let named = u32::from_le_bytes([h[16], h[17], h[18], h[19]]);
    let body = buf.get(HDR_LEN..)?;
    (u32::try_from(body.len()) == Ok(named)).then_some((request(h), body))
}

/// The request a frame `parse` refused is answered under: its caller waits
/// in its call for a reply, so a refusal is answered too, under what the
/// frame names, or zeros when it is too short to name anything.
pub fn refused(buf: &[u8]) -> Request {
    buf.first_chunk::<HDR_LEN>().map_or(Request { op: 0, flags: 0, request_id: 0 }, request)
}

fn request(h: &[u8; HDR_LEN]) -> Request {
    Request {
        op: u16::from_le_bytes([h[6], h[7]]),
        flags: u16::from_le_bytes([h[8], h[9]]),
        request_id: u32::from_le_bytes([h[12], h[13], h[14], h[15]]),
    }
}
