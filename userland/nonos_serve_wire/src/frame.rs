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

//! One frame: its header read and written, and a question held to what its
//! op may carry. `len` is never trusted: it must name exactly the bytes after
//! the header, and no more than MAX_BODY.

use crate::bodies::{read_asked, written};
use crate::ops::{CLOSE, HEADER, MAGIC, MAX_BODY, OPEN, READ, WRITE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    pub op: u8,
    pub status: u8,
    pub conn: u16,
    pub len: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    Short,
    Magic,
    Op,
    Len,
    Body,
}

/// Any frame, asked or answered: the header and the body after it.
pub fn decode(frame: &[u8]) -> Result<(Header, &[u8]), Refused> {
    let head = frame.get(..HEADER).ok_or(Refused::Short)?;
    if head[..4] != MAGIC {
        return Err(Refused::Magic);
    }
    let len = u32::from_le_bytes([head[8], head[9], head[10], head[11]]);
    let h =
        Header { op: head[4], status: head[5], conn: u16::from_le_bytes([head[6], head[7]]), len };
    if !matches!(h.op, OPEN | WRITE | READ | CLOSE) {
        return Err(Refused::Op);
    }
    let body = &frame[HEADER..];
    if len as usize > MAX_BODY || body.len() != len as usize {
        return Err(Refused::Len);
    }
    Ok((h, body))
}

/// A question: status 0, and the body its op carries (the table in lib.rs).
pub fn request(frame: &[u8]) -> Result<(Header, &[u8]), Refused> {
    let (h, body) = decode(frame)?;
    let fits = match h.op {
        OPEN | CLOSE => body.is_empty(),
        WRITE => written(body).is_some(),
        _ => read_asked(body).is_some(),
    };
    (h.status == 0 && fits).then_some((h, body)).ok_or(Refused::Body)
}
