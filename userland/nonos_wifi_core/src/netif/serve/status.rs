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

//! The status-only responses: the header and an i32 status, with or
//! without one trailing byte.

use super::*;

// Response: header + i32 status.
pub(super) fn status_only(out: &mut [u8], op: u16, rid: u32, status: i32) -> Option<usize> {
    let n = write_response_header(out, op, rid, 4)?;
    put_i32(out, n, status)?;
    Some(n + 4)
}

// Response: header + i32 status + one trailing byte (link up flag).
pub(super) fn status_byte(
    out: &mut [u8],
    op: u16,
    rid: u32,
    status: i32,
    byte: u8,
) -> Option<usize> {
    let n = write_response_header(out, op, rid, 5)?;
    put_i32(out, n, status)?;
    *out.get_mut(n + 4)? = byte;
    Some(n + 5)
}
