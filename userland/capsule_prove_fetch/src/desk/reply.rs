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

//! The relay's answer: its head, then for a GET it found exactly its
//! Content-Length, at most 1 MiB; chunked, unsized or longer is refused.

use alloc::vec::Vec;

use super::head::{length, Refused};

pub const HEAD_MAX: usize = 8 * 1024;

#[derive(PartialEq, Eq, Debug)]
pub enum Read {
    More,
    /// The status code, and the body when one was wanted and came.
    Done(u16, Vec<u8>),
}

pub struct Reply {
    want_body: bool,
    raw: Vec<u8>,
}

impl Reply {
    pub fn new(want_body: bool) -> Reply {
        Reply { want_body, raw: Vec::new() }
    }

    pub fn feed(&mut self, b: &[u8]) -> Result<Read, Refused> {
        self.raw.extend_from_slice(b);
        let Some(end) = self.raw.windows(4).position(|w| w == b"\r\n\r\n") else {
            return if self.raw.len() > HEAD_MAX { Err(Refused) } else { Ok(Read::More) };
        };
        let head = core::str::from_utf8(&self.raw[..end]).map_err(|_| Refused)?;
        let mut lines = head.split("\r\n");
        let code = lines.next().and_then(|s| s.strip_prefix("HTTP/1.")).and_then(|s| s.get(2..5));
        let code: u16 = code.ok_or(Refused)?.parse().map_err(|_| Refused)?;
        if !(self.want_body && code == 200) {
            return Ok(Read::Done(code, Vec::new()));
        }
        let len = length(lines)?;
        let body = &self.raw[end + 4..];
        match body.len() {
            n if n < len => Ok(Read::More),
            n if n == len => Ok(Read::Done(code, body.to_vec())),
            _ => Err(Refused),
        }
    }
}
