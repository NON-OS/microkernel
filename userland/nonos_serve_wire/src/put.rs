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

//! A frame written: the header from `h` with `len` set to the body's length.

use crate::frame::Header;
use crate::ops::{HEADER, MAGIC, MAX_BODY};

/// `h` and `body` written into `out`, `h.len` set to the body's length; the
/// bytes written, or None when the body is past MAX_BODY or `out` too short.
pub fn put(out: &mut [u8], h: Header, body: &[u8]) -> Option<usize> {
    let n = HEADER + body.len();
    if body.len() > MAX_BODY || out.len() < n {
        return None;
    }
    out[..4].copy_from_slice(&MAGIC);
    out[4] = h.op;
    out[5] = h.status;
    out[6..8].copy_from_slice(&h.conn.to_le_bytes());
    out[8..12].copy_from_slice(&(body.len() as u32).to_le_bytes());
    out[HEADER..n].copy_from_slice(body);
    Some(n)
}
