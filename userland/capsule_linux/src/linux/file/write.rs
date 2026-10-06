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

/*
 * `write` on a file: at the descriptor's offset, or at the end when it
 * was opened O_APPEND, and the offset moves to where the write ended.
 */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::rw::{write_at, MAX_IO};

pub fn write(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let at = guest.fds.get(fd as usize).map_or(0, super::desc::pos);
    let (n, end) = match whole(guest, fd, buf, len, at) {
        Ok(done) => done,
        Err(e) => return errno::fail(e),
    };
    if let Some(entry) = guest.fds.get_mut(fd as usize) {
        super::desc::set_pos(entry, end);
    }
    errno::ok(n)
}

/*
 * All `len` bytes at `buf` into the file at `at`, as a Linux file takes a
 * whole write: MAX_IO bounds one copy out of the guest, not the call. The
 * count written and where the write ended; a failure after some bytes
 * landed is the count so far, as on Linux.
 */
pub fn whole(guest: &mut Guest, fd: u64, buf: u64, len: u64, at: u64) -> Result<(u64, u64), i64> {
    let (mut done, mut end) = (0u64, at);
    loop {
        let take = ((len - done) as usize).min(MAX_IO);
        let got = match guest.read(buf + done, take) {
            Some(bytes) => write_at(guest, fd, end, &bytes),
            None => Err(errno::EFAULT),
        };
        match got {
            Ok((0, _)) if take > 0 => return Ok((done, end)),
            Ok((n, to)) => (done, end) = (done + n as u64, to),
            Err(e) if done == 0 => return Err(e),
            Err(_) => return Ok((done, end)),
        }
        if done >= len {
            return Ok((done, end));
        }
    }
}
