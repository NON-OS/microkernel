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
//! Who the guest is, and which process group it belongs to.

use crate::linux::abi::errno;
use crate::linux::file;
use crate::linux::guest::Guest;

/* The identity every guest runs as. */
const GUEST_UID: u64 = 0;

/*
 * The process that forked this one, as /proc/<pid>/stat names it; the
 * personality, the namespace's pid 1, for the program it started. A kernel
 * pid: the serve loop gives it the number the namespace knows it by.
 */
pub fn getppid(guest: &Guest) -> u64 {
    let parent = file::view_with(|v| {
        let me = v.procs.iter().find(|p| p.kernel == guest.pid)?;
        v.procs.iter().find(|p| p.ns == me.ppid).map(|p| p.kernel)
    });
    errno::ok(u64::from(parent.unwrap_or(guest.parent)))
}

/*
 * Setting the identity to the one already held is the only change
 * that can be honoured, so it is the only one accepted.
 */
pub fn setuid(want: u64) -> u64 {
    match want {
        GUEST_UID => errno::ok(0),
        _ => errno::fail(errno::EPERM),
    }
}
