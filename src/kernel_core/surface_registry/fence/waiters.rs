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

//! Who waits on which slot's fence: the list behind one lock, under the
//! rules of `waiters_list`.

use alloc::vec::Vec;

use spin::Mutex;

use super::waiters_list::{arm_in, disarm_in, forget_in, take_in};

static WAITERS: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());

pub(super) fn arm(idx: u32, pid: u32) {
    arm_in(&mut WAITERS.lock(), idx, pid);
}

pub(super) fn disarm(idx: u32, pid: u32) {
    disarm_in(&mut WAITERS.lock(), idx, pid);
}

pub(super) fn take(idx: u32) -> Vec<u32> {
    take_in(&mut WAITERS.lock(), idx)
}

/// A torn-down thread waits on nothing (process/exit/forget.rs).
pub fn forget(pid: u32) {
    forget_in(&mut WAITERS.lock(), pid);
}
