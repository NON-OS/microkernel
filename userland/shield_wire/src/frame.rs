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

//! A request or a reply on the wire: an eight-byte head, then the text.

use alloc::vec::Vec;

pub use crate::frame_reply::{decode_reply, encode_reply, Reply};

/// Sequence number (4), operation (2), reserved (2).
pub(crate) const HEAD: usize = 8;

/// The longest message either side reads, head included.
pub const MESSAGE_MAX: usize = 16 * 1024;
/// The longest reply body that arrives whole.
pub const BODY_MAX: usize = MESSAGE_MAX - HEAD;

pub struct Request<'a> {
    pub seq: u32,
    pub op: u16,
    /// The fields, one per line.
    pub body: &'a str,
}

pub fn encode_request(seq: u32, op: u16, fields: &[&str]) -> Vec<u8> {
    let text = fields.iter().fold(0usize, |t, f| t.saturating_add(f.len()).saturating_add(1));
    let mut out = Vec::with_capacity(text.saturating_add(HEAD));
    out.extend_from_slice(&seq.to_le_bytes());
    out.extend_from_slice(&op.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    for (i, f) in fields.iter().enumerate() {
        if i > 0 {
            out.push(b'\n');
        }
        out.extend_from_slice(f.as_bytes());
    }
    out
}

pub fn decode_request(bytes: &[u8]) -> Option<Request<'_>> {
    let head = bytes.get(..HEAD)?;
    let body = core::str::from_utf8(bytes.get(HEAD..)?).ok()?;
    Some(Request {
        seq: u32::from_le_bytes(head.get(..4)?.try_into().ok()?),
        op: u16::from_le_bytes(head.get(4..6)?.try_into().ok()?),
        body,
    })
}
