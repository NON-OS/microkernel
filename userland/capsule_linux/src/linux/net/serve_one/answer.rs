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

//! An answer on the wire: the question's op and conn, a status, a body.

use alloc::vec;
use alloc::vec::Vec;

use nonos_serve_wire::{put, Header, HEADER};

/// The answer frame to the question `h`. A body is never past MAX_BODY
/// here: READ gives at most `want`, which the codec holds to it.
pub fn answer(h: Header, status: u8, body: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; HEADER + body.len()];
    let n = put(&mut out, Header { status, ..h }, body).unwrap_or(0);
    out.truncate(n);
    out
}

/// The question a frame the codec refused was, as far as it has its op
/// and conn, so its refusal goes back under them; an op the wire does not
/// know goes back as it came.
pub fn refused(frame: &[u8]) -> Header {
    let at = |i: usize| frame.get(i).copied().unwrap_or(0);
    Header { op: at(4), status: 0, conn: u16::from_le_bytes([at(6), at(7)]), len: 0 }
}
