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
 * Each process's nice value, as setpriority keeps it and getpriority
 * reports it. NONOS schedules the family's threads without reading it.
 */

use alloc::vec::Vec;
use core::cell::RefCell;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::who::named;

pub(super) struct Nice(pub(super) RefCell<Vec<(u32, i64)>>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Nice {}

static NICE: Nice = Nice(RefCell::new(Vec::new()));

pub fn nice_of(kernel: u32) -> i64 {
    NICE.0.borrow().iter().find(|(p, _)| *p == kernel).map_or(0, |(_, n)| *n)
}

/* The raw syscall answers 20 - nice, so that no success is negative. */
pub fn getpriority(guest: &Guest, which: u64, who: u64) -> u64 {
    match named(guest, which, who) {
        Ok(all) => errno::ok((20 - all.iter().map(|k| nice_of(*k)).min().unwrap_or(0)) as u64),
        Err(e) => errno::fail(e),
    }
}

/* Without CAP_SYS_NICE a process may only lower its priority: raise nice. */
pub fn setpriority(guest: &Guest, which: u64, who: u64, value: u64) -> u64 {
    let want = (value as i32 as i64).clamp(-20, 19);
    let all = match named(guest, which, who) {
        Ok(all) => all,
        Err(e) => return errno::fail(e),
    };
    if all.iter().any(|k| want < nice_of(*k)) {
        return errno::fail(errno::EACCES);
    }
    let mut table = NICE.0.borrow_mut();
    for k in all {
        table.retain(|(p, _)| *p != k);
        table.push((k, want));
    }
    errno::ok(0)
}
