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

/* fallocate, as tmpfs answers it. */

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::super::cache;
use super::super::size::resize;
use super::zero::zero;

const KEEP_SIZE: u64 = 0x01;

const PUNCH_HOLE: u64 = 0x02;

/* FALLOC_FL_SUPPORTED_MASK: every mode bit Linux knows. */
const KNOWN: u64 = 0x7f;

/*
 * As Linux's tmpfs does it, since the family's writable directories are
 * its tmpfs mounts: mode 0 makes the file at least `at + len` long, the new
 * bytes zero; KEEP_SIZE alone has nothing to allocate; PUNCH_HOLE with
 * KEEP_SIZE zeroes the range. tmpfs refuses every other mode.
 */
pub fn fallocate(guest: &mut Guest, fd: u64, mode: u64, at: u64, len: u64) -> u64 {
    if (at as i64) < 0 || (len as i64) <= 0 {
        return errno::fail(errno::EINVAL);
    }
    if mode & !KNOWN != 0 {
        return errno::fail(errno::EOPNOTSUPP);
    }
    /* Linux's vfs_fallocate: a hole may only be punched inside the size. */
    if mode & PUNCH_HOLE != 0 && mode & KEEP_SIZE == 0 {
        return errno::fail(errno::EOPNOTSUPP);
    }
    let Some(entry) = guest.fds.get(fd as usize).filter(|f| f.is_open()) else {
        return errno::fail(errno::EBADF);
    };
    match entry.kind {
        Kind::Pipe => return errno::fail(errno::ESPIPE),
        Kind::File if !entry.writable => return errno::fail(errno::EBADF),
        Kind::File if !super::super::super::synth::owns(&entry.path) => {}
        _ => return errno::fail(errno::ENODEV),
    }
    if mode & !(KEEP_SIZE | PUNCH_HOLE) != 0 {
        return errno::fail(errno::EOPNOTSUPP);
    }
    let path = entry.path.clone();
    let now = cache::size(&path).unwrap_or(entry.size);
    let want = at.saturating_add(len);
    match mode {
        /* The descriptor learns the new length only once the copy has it. */
        0 if want > now => {
            let done = resize(&path, want, true);
            if let Some(e) = guest.fds.get_mut(fd as usize).filter(|_| done == errno::ok(0)) {
                e.size = want;
            }
            done
        }
        m if m & PUNCH_HOLE != 0 && at < now => zero(&path, at, len),
        _ => errno::ok(0),
    }
}
