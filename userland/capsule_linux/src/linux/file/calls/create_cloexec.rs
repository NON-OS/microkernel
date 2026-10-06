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
 * The calls that make a descriptor and may ask in the same call for it to
 * close on exec: Python's selectors open their epoll that way, and
 * os.memfd_create its memfd, so a child it starts keeps neither.
 */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::create_flags::{epoll_flags, memfd_flags};

pub fn epoll_create1(guest: &mut Guest, flags: u64) -> u64 {
    match epoll_flags(flags) {
        Ok(cloexec) => {
            let got = crate::linux::file::epoll_create(guest);
            cloexec_on(guest, got, cloexec)
        }
        Err(e) => errno::fail(e),
    }
}

pub fn new_memfd(guest: &mut Guest, flags: u64) -> u64 {
    match memfd_flags(flags) {
        Ok(cloexec) => {
            let got = crate::linux::file::memfd_create(guest);
            cloexec_on(guest, got, cloexec)
        }
        Err(e) => errno::fail(e),
    }
}

/* The flag on the descriptor `got` names, when the call made one. */
fn cloexec_on(guest: &mut Guest, got: u64, cloexec: bool) -> u64 {
    if (got as i64) >= 0 {
        if let Some(fd) = guest.fds.get_mut(got as usize) {
            fd.cloexec = cloexec;
        }
    }
    got
}
