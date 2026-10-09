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

/// Sequence number (4), operation (2), reserved (2).
const HEAD: usize = 8;

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

pub struct Reply<'a> {
    pub seq: u32,
    pub status: i32,
    /// `name=value` lines.
    pub body: &'a str,
}

pub fn encode_request(seq: u32, op: u16, fields: &[&str]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEAD + fields.iter().map(|f| f.len() + 1).sum::<usize>());
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
        seq: u32::from_le_bytes([head[0], head[1], head[2], head[3]]),
        op: u16::from_le_bytes([head[4], head[5]]),
        body,
    })
}

pub fn encode_reply(seq: u32, status: i32, body: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEAD + body.len());
    out.extend_from_slice(&seq.to_le_bytes());
    out.extend_from_slice(&status.to_le_bytes());
    out.extend_from_slice(body.as_bytes());
    out
}

pub fn decode_reply(bytes: &[u8]) -> Option<Reply<'_>> {
    let head = bytes.get(..HEAD)?;
    let body = core::str::from_utf8(bytes.get(HEAD..)?).ok()?;
    Some(Reply {
        seq: u32::from_le_bytes([head[0], head[1], head[2], head[3]]),
        status: i32::from_le_bytes([head[4], head[5], head[6], head[7]]),
        body,
    })
}
