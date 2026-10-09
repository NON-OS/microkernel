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
 * dup3: dup2 with the copy's close-on-exec flag chosen in the same call, as
 * Go's forkExec does when it moves descriptors out of the way of the ones
 * the child is to have, so the moved copies are gone after its exec.
 */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::dup3_args::dup3_args;

pub fn dup3(guest: &mut Guest, from: u64, to: u64, flags: u64) -> u64 {
    let cloexec = match dup3_args(from, to, flags) {
        Ok(cloexec) => cloexec,
        Err(e) => return errno::fail(e),
    };
    let got = crate::linux::call::dup2(guest, from, to);
    /* dup2 leaves the copy without the flag, as Linux's dup2 does. */
    if (got as i64) >= 0 {
        if let Some(fd) = guest.fds.get_mut(to as usize) {
            fd.cloexec = cloexec;
        }
    }
    got
}
