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

//! CLONE_PIDFD: a pidfd for the child a clone made, its number written where
//! the clone's parent_tid points, as an int, as Linux writes it.

use crate::linux::abi::errno;
use crate::linux::file::MAX_FDS;
use crate::linux::guest::{Fd, Guest, Kind};

/// The number the pidfd will take, the lowest free as `install` picks it,
/// written at `at` before any child is made: a word that cannot be written
/// is EFAULT and a full table EMFILE, with no child made, as copy_process
/// refuses both before its child runs.
pub fn reserve(guest: &Guest, at: u64) -> Result<usize, i64> {
    let slot = guest.fds.iter().position(|f| !f.is_open()).unwrap_or(guest.fds.len());
    if slot >= MAX_FDS {
        return Err(errno::EMFILE);
    }
    if guest.write(at, &(slot as i32).to_le_bytes()) < 4 {
        return Err(errno::EFAULT);
    }
    Ok(slot)
}

/// The pidfd for `child`, at the number `reserve` wrote, close-on-exec as
/// Linux opens it. Making the child opens no descriptor here, so it is free.
pub fn give(guest: &mut Guest, slot: usize, child: u32) {
    let mut fd = Fd::empty(Kind::Pidfd);
    fd.handle = child;
    fd.cloexec = true;
    match guest.fds.get_mut(slot) {
        Some(free) => *free = fd,
        None => guest.fds.push(fd),
    }
}
