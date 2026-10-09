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

/* Which processes a priority call names: one, a group, or a user's. */

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::file;
use crate::linux::guest::Guest;

const PRIO_PROCESS: u64 = 0;

const PRIO_PGRP: u64 = 1;

const PRIO_USER: u64 = 2;

/* The processes `which` and `who` name, by kernel pid, among the family's. */
pub(super) fn named(guest: &Guest, which: u64, who: u64) -> Result<Vec<u32>, i64> {
    let who = who as u32;
    let found: Vec<u32> = file::view_with(|v| match which {
        PRIO_PROCESS if who == 0 => alloc::vec![guest.pid],
        PRIO_PROCESS => v.procs.iter().filter(|p| p.ns == who).map(|p| p.kernel).collect(),
        PRIO_PGRP => {
            let group = if who == 0 { v.find(v.me).map_or(0, |p| p.pgid) } else { who };
            v.procs.iter().filter(|p| p.pgid == group).map(|p| p.kernel).collect()
        }
        /* Every process of the family is root's. */
        PRIO_USER if who == 0 => v.procs.iter().map(|p| p.kernel).collect(),
        _ => Vec::new(),
    });
    match which {
        PRIO_PROCESS | PRIO_PGRP | PRIO_USER if found.is_empty() => Err(errno::ESRCH),
        PRIO_PROCESS | PRIO_PGRP | PRIO_USER => Ok(found),
        _ => Err(errno::EINVAL),
    }
}
