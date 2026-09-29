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


//! A guest's console output, carried to the host's log.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// Cap on one transfer, matching the kernel's own peer-copy ceiling.
const MAX_IO: u64 = 1 << 20;

/* The longest line the kernel's debug channel takes; it refuses a longer one
 * whole (src/syscall/microkernel/debug.rs MAX_LEN). */
const MAX_LINE: usize = 256;

/// A guest's console output, carried to the host's log. The bytes are the
/// guest's and are never interpreted, only forwarded.
/*
 * Forwarded in pieces the kernel takes. The count returned is what was
 * carried: a write the log refuses part way is a short write, as Linux
 * reports one, never a claimed success.
 */
pub(super) fn console(guest: &Guest, buf: u64, len: u64) -> u64 {
    if len == 0 {
        return errno::ok(0);
    }
    let take = len.min(MAX_IO);
    let Some(bytes) = guest.read(buf, take as usize) else {
        return errno::fail(errno::EFAULT);
    };
    let mut done = 0;
    for piece in bytes.chunks(MAX_LINE) {
        if nonos_libc::mk_debug(piece.as_ptr(), piece.len()) < 0 {
            break;
        }
        done += piece.len();
    }
    match done {
        0 => errno::fail(errno::EIO),
        n => errno::ok(n as u64),
    }
}

