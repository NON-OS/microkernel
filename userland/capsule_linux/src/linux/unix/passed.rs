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


//! The descriptors a sendmsg passes, kept as the server's own. libwayland
//! closes each one it sent once the sendmsg has gone; a request that waits
//! (the server backlogged, or a write taken in part) would find the number
//! closed or given to whatever the client opened next, and a pool would
//! map the wrong memory. Each is duplicated at the sendmsg, into a slot the
//! client never named, and closed by whatever takes it once done with it.

use alloc::vec::Vec;

use crate::linux::file::install;
use crate::linux::guest::{Fd, Guest};

/// A private descriptor onto each of `fds`; None, making none, when one is
/// not open (EBADF, as Linux has it).
pub fn kept(guest: &mut Guest, fds: &[u32]) -> Option<Vec<u32>> {
    let open = |fd: &u32| guest.fds.get(*fd as usize).is_some_and(|f| f.is_open());
    if !fds.iter().all(open) {
        return None;
    }
    let mut out = Vec::with_capacity(fds.len());
    for &fd in fds {
        let Some(mut copy) = guest.fds.get(fd as usize).map(Fd::clone_of) else { break };
        copy.cloexec = true;
        match install(guest, copy) {
            Some(slot) => out.push(slot as u32),
            None => break,
        }
    }
    if out.len() != fds.len() {
        out.iter().for_each(|&fd| drop(guest, fd));
        return None;
    }
    Some(out)
}

/// A kept descriptor, done with.
pub fn drop(guest: &mut Guest, fd: u32) {
    let _ = crate::linux::file::close(guest, u64::from(fd));
}
