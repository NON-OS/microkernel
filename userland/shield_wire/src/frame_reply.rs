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

//! A reply on the wire: the eight-byte head (sequence, status), then the
//! `name=value` lines.

use alloc::vec::Vec;

use crate::frame::HEAD;

pub struct Reply<'a> {
    pub seq: u32,
    pub status: i32,
    /// `name=value` lines.
    pub body: &'a str,
}

pub fn encode_reply(seq: u32, status: i32, body: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(body.len().saturating_add(HEAD));
    out.extend_from_slice(&seq.to_le_bytes());
    out.extend_from_slice(&status.to_le_bytes());
    out.extend_from_slice(body.as_bytes());
    out
}

pub fn decode_reply(bytes: &[u8]) -> Option<Reply<'_>> {
    let head = bytes.get(..HEAD)?;
    let body = core::str::from_utf8(bytes.get(HEAD..)?).ok()?;
    Some(Reply {
        seq: u32::from_le_bytes(head.get(..4)?.try_into().ok()?),
        status: i32::from_le_bytes(head.get(4..8)?.try_into().ok()?),
        body,
    })
}
