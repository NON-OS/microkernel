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

/* times, and what the calling process and its waited-for children used. */

use crate::linux::abi::errno;
use crate::linux::file::{self, cpu::Usage, declared};
use crate::linux::guest::Guest;

use super::super::super::family_ms;

pub(super) const TICK_MS: u64 = 1000 / declared::HZ;

/*
 * times: the process's and its waited-for children's ticks, and the
 * ticks since the family started.
 */
pub fn times(guest: &Guest, out: u64) -> u64 {
    if out != 0 {
        let (me, kids) = (mine(guest), children(guest));
        let mut b = [0u8; 32];
        for (i, v) in [me.user, me.system, kids.user, kids.system].iter().enumerate() {
            b[i * 8..i * 8 + 8].copy_from_slice(&v.to_le_bytes());
        }
        if guest.write(out, &b) != 32 {
            return errno::fail(errno::EFAULT);
        }
    }
    errno::ok(family_ms() / TICK_MS)
}

/* Every thread of the calling process. */
pub fn mine(guest: &Guest) -> Usage {
    let mut all = alloc::vec![guest.pid];
    all.extend_from_slice(&guest.threads);
    let mut u = file::cpu::usage(&all);
    u.resident_kb = file::cpu::usage(&[guest.pid]).resident_kb;
    u
}

pub(super) fn children(guest: &Guest) -> Usage {
    file::cpu::children(guest.pid, |c| !guest.children.contains(&c))
}
