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

//! The scheduler calls, answered as Linux answers an unprivileged process
//! on the one CPU the guest is shown. Scheduling is the kernel's: a policy a
//! guest names changes nothing, and a real-time one is refused, since no
//! guest holds the privilege Linux asks for it.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

const SCHED_OTHER: u64 = 0;
const SCHED_FIFO: u64 = 1;
const SCHED_RR: u64 = 2;
const SCHED_BATCH: u64 = 3;
const SCHED_IDLE: u64 = 5;
const SCHED_RESET_ON_FORK: u64 = 0x4000_0000;

/// 0 is the caller; anything else must be one of the guest's own threads.
fn own(guest: &Guest, pid: u64) -> bool {
    pid == 0 || guest.owns(pid as u32)
}

pub fn sched_getscheduler(guest: &Guest, pid: u64) -> u64 {
    match own(guest, pid) {
        true => errno::ok(SCHED_OTHER),
        false => errno::fail(errno::ESRCH),
    }
}

pub fn sched_setscheduler(guest: &Guest, pid: u64, policy: u64, param: u64) -> u64 {
    if !own(guest, pid) {
        return errno::fail(errno::ESRCH);
    }
    let Some(priority) = priority(guest, param) else {
        return errno::fail(if param == 0 { errno::EINVAL } else { errno::EFAULT });
    };
    match policy & !SCHED_RESET_ON_FORK {
        SCHED_OTHER | SCHED_BATCH | SCHED_IDLE if priority == 0 => errno::ok(0),
        // The priority is checked before the privilege, as Linux orders them.
        SCHED_FIFO | SCHED_RR if (1..=99).contains(&priority) => errno::fail(errno::EPERM),
        _ => errno::fail(errno::EINVAL),
    }
}

pub fn sched_getparam(guest: &Guest, pid: u64, param: u64) -> u64 {
    if !own(guest, pid) {
        return errno::fail(errno::ESRCH);
    }
    match guest.write(param, &0i32.to_le_bytes()) == 4 {
        true => errno::ok(0),
        false => errno::fail(errno::EFAULT),
    }
}

/// Under SCHED_OTHER the only priority is zero.
pub fn sched_setparam(guest: &Guest, pid: u64, param: u64) -> u64 {
    if !own(guest, pid) {
        return errno::fail(errno::ESRCH);
    }
    match priority(guest, param) {
        Some(0) => errno::ok(0),
        Some(_) => errno::fail(errno::EINVAL),
        None => errno::fail(if param == 0 { errno::EINVAL } else { errno::EFAULT }),
    }
}

/// `sched_get_priority_max` and `_min`: the range each policy has on Linux.
pub fn priority_bound(policy: u64, max: bool) -> u64 {
    match policy {
        SCHED_FIFO | SCHED_RR => errno::ok(if max { 99 } else { 1 }),
        SCHED_OTHER | SCHED_BATCH | SCHED_IDLE => errno::ok(0),
        _ => errno::fail(errno::EINVAL),
    }
}

/// Any mask that includes the one CPU is accepted; one that leaves it out
/// leaves the thread nowhere to run.
pub fn sched_setaffinity(guest: &Guest, pid: u64, size: u64, mask: u64) -> u64 {
    if !own(guest, pid) {
        return errno::fail(errno::ESRCH);
    }
    match (size, guest.read(mask, 1)) {
        (0, _) => errno::fail(errno::EINVAL),
        (_, None) => errno::fail(errno::EFAULT),
        (_, Some(first)) if first[0] & 1 == 0 => errno::fail(errno::EINVAL),
        _ => errno::ok(0),
    }
}

fn priority(guest: &Guest, param: u64) -> Option<i32> {
    let raw = guest.read(param, 4).filter(|_| param != 0)?;
    Some(i32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]))
}
