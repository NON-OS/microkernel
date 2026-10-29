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


//! The data half of a msghdr: the iovecs, joined.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::msg::u64le;

/// The most one message puts on the connection, as a stream socket's send
/// buffer takes no more at once; the rest is a short write the client sends
/// again.
const MOST: usize = 1 << 20;

/// The iovecs, joined, up to MOST. Each is a pointer and a length, sixteen
/// bytes.
pub(super) fn gather(guest: &Guest, iov: u64, count: u64) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    for i in 0..count.min(64) {
        let entry = guest.read(iov.checked_add(i * 16)?, 16)?;
        let base = u64le(&entry, 0);
        let room = MOST - out.len();
        let len = u64le(&entry, 8).min(room as u64);
        if len == 0 {
            continue;
        }
        out.extend_from_slice(&guest.read(base, len as usize)?);
    }
    Some(out)
}
