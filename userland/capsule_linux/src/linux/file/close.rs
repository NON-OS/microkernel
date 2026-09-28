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

/* Closing a descriptor, and writing out anything it was holding. */

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind};

pub fn close(guest: &mut Guest, fd: u64) -> u64 {
    let Some(entry) = guest.fds.get(fd as usize) else {
        return errno::fail(errno::EBADF);
    };
    if !entry.is_open() {
        return errno::fail(errno::EBADF);
    }
    if let Some(d) = super::desc::of(entry).filter(|d| !super::desc::held_elsewhere(guest, fd, *d))
    {
        super::desc::gone(d);
    }
    let flushed = flush(guest, fd);
    /*
     * The store handle is dropped with the descriptor, which closes it on the
     * server.
     */
    guest.fds[fd as usize] = Fd::empty(Kind::Free);
    super::epoll::forget(guest, fd);
    match flushed {
        Ok(()) => errno::ok(0),
        Err(e) => errno::fail(e),
    }
}

/*
 * A written file's bytes to the store. The family's copy is let go when
 * this process holds no other descriptor on it; another process that
 * still does reads the store, which now has every byte.
 */
pub(super) fn flush(guest: &Guest, fd: u64) -> Result<(), i64> {
    let Some(entry) = guest.fds.get(fd as usize) else {
        return Ok(());
    };
    if entry.kind != Kind::File || !entry.writable || super::synth::owns(&entry.path) {
        return Ok(());
    }
    let path = &entry.path;
    let others = guest
        .fds
        .iter()
        .enumerate()
        .any(|(i, f)| i as u64 != fd && f.kind == Kind::File && f.path == *path);
    super::cache::flush(path, others)
}
