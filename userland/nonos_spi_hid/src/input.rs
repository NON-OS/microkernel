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
//! An input report's header read, and the body that follows it: a header
//! with the wrong version or sync byte is no report, and a body is taken
//! only whole and within the length its header gave.

use super::facts::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Header {
    /// Body bytes to read: the header's words times four.
    pub body_len: usize,
    pub last: bool,
}

pub fn header(b: [u8; HEADER_LEN]) -> Option<Header> {
    let h = u32::from_le_bytes(b);
    if h & 0xF != HEADER_VERSION || b[3] != HEADER_SYNC {
        return None;
    }
    let words = (h >> LENGTH_SHIFT & LENGTH_MASK) as usize;
    Some(Header { body_len: words * 4, last: h & LAST_FRAGMENT != 0 })
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Body<'a> {
    pub kind: u8,
    pub id: u8,
    pub content: &'a [u8],
}

/// The report in `body`, whose content length must fit what came.
pub fn body(body: &[u8]) -> Option<Body<'_>> {
    let h = body.get(..BODY_HEAD)?;
    let len = u16::from_le_bytes([h[1], h[2]]) as usize;
    let content = body.get(BODY_HEAD..BODY_HEAD + len)?;
    Some(Body { kind: h[0], id: h[3], content })
}
