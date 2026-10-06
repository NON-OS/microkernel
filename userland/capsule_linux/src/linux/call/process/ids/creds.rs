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
 * The ids a guest runs as, which are root's; its groups, which are none;
 * and its execution domain, which is Linux's.
 */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/* getresuid and getresgid: real, effective and saved, all 0. */
pub fn getres(guest: &Guest, ids: [u64; 3]) -> u64 {
    for at in ids {
        if guest.write(at, &0u32.to_le_bytes()) != 4 {
            return errno::fail(errno::EFAULT);
        }
    }
    errno::ok(0)
}

/* setresuid and setresgid: -1 keeps an id, 0 is the one it has. */
pub fn setres(ids: [u64; 3]) -> u64 {
    match ids.iter().all(|id| *id as u32 == u32::MAX || *id as u32 == 0) {
        true => errno::ok(0),
        false => errno::fail(errno::EPERM),
    }
}

/* No supplementary groups. */
pub fn getgroups(size: u64) -> u64 {
    match (size as i32) < 0 {
        true => errno::fail(errno::EINVAL),
        false => errno::ok(0),
    }
}

/* Changing groups needs CAP_SETGID, which no guest holds. */
pub fn setgroups() -> u64 {
    errno::fail(errno::EPERM)
}

const PER_LINUX: u64 = 0;

const QUERY: u64 = 0xffff_ffff;

/* Only asking is served: every guest runs as PER_LINUX. */
pub fn personality(persona: u64) -> u64 {
    match persona & 0xffff_ffff {
        QUERY | PER_LINUX => errno::ok(PER_LINUX),
        _ => {
            let line = b"[LINUX] refused personality: every guest runs as PER_LINUX\n";
            let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
            errno::fail(errno::EINVAL)
        }
    }
}
