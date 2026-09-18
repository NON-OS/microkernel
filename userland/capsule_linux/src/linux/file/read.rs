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


//! Reading from a file a guest has open.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

/// One transfer, matching the kernel's own peer-copy ceiling.
const MAX_IO: u64 = 1 << 20;

pub fn read(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let Some(entry) = guest.fds.get_mut(fd as usize) else {
        return errno::fail(errno::EBADF);
    };
    if entry.kind != Kind::File {
        return errno::fail(errno::EBADF);
    }
    let Some(stream) = entry.stream.as_mut() else {
        /* Opened to write only: Linux answers EBADF, not end of file. */
        return errno::fail(errno::EBADF);
    };
    if len == 0 || entry.offset >= entry.size {
        return errno::ok(0);
    }
    let want = len.min(MAX_IO).min(entry.size - entry.offset);
    let Ok(bytes) = stream.read_window(entry.offset, want as u32) else {
        return errno::fail(errno::EIO);
    };
    if bytes.is_empty() {
        return errno::ok(0);
    }
    let wrote = guest.write(buf, &bytes);
    if wrote < bytes.len() as i64 {
        return errno::fail(errno::EFAULT);
    }
    entry.offset += bytes.len() as u64;
    errno::ok(bytes.len() as u64)
}
