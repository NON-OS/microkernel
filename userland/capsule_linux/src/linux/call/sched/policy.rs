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

//! sched_getscheduler and sched_setscheduler, as Linux answers an
//! unprivileged process.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

pub(super) const SCHED_OTHER: u64 = 0;
pub(super) const SCHED_FIFO: u64 = 1;
pub(super) const SCHED_RR: u64 = 2;
pub(super) const SCHED_BATCH: u64 = 3;
pub(super) const SCHED_IDLE: u64 = 5;
const SCHED_RESET_ON_FORK: u64 = 0x4000_0000;

/// 0 is the caller; anything else must be one of the guest's own threads.
pub(super) fn own(guest: &Guest, pid: u64) -> bool {
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

pub(super) fn priority(guest: &Guest, param: u64) -> Option<i32> {
    let raw = guest.read(param, 4).filter(|_| param != 0)?;
    Some(i32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]))
}
