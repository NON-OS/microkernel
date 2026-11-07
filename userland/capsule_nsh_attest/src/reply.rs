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

//! A reply frame: the shared header, then the body. An error reply carries
//! the op it answers and an empty body, so the front learns both what failed
//! and for which request.

use alloc::vec;
use alloc::vec::Vec;
use nsh_front_wire::{write_header, HDR_LEN, ST_OVERFLOW};

/// A frame answering `op` with `status` and `body`. `write_header` lays a
/// header down only where the whole frame fits, so the frame is sized for
/// the body before it is called; a body too long for any frame is answered
/// `ST_OVERFLOW` with no body.
pub fn frame(op: u16, status: u16, body: &[u8]) -> Vec<u8> {
    let Some(total) = HDR_LEN.checked_add(body.len()) else {
        return err(op, ST_OVERFLOW);
    };
    let mut out = vec![0u8; total];
    match write_header(&mut out, op, status, body.len()) {
        Some(_) => {
            out[HDR_LEN..].copy_from_slice(body);
            out
        }
        None => err(op, ST_OVERFLOW),
    }
}

/// An error reply for `op` with `status`: the header alone.
pub fn err(op: u16, status: u16) -> Vec<u8> {
    let mut out = vec![0u8; HDR_LEN];
    // A header and no body is exactly HDR_LEN bytes, which `out` holds.
    let _ = write_header(&mut out, op, status, 0);
    out
}
