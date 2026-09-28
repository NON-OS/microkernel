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


//! One-way death notices from the kernel to a guest's supervisor.
//!
//! A guest thread that ends on a signal cannot park and wait for a reply, and
//! the kernel does not decide what a dead thread means: that is the
//! supervisor's, and its personality's, policy. So the kernel leaves a notice
//! its supervisor collects on the next wait, keyed by the supervisor so it
//! survives the dead thread's teardown, and one-shot because no reply follows.

use alloc::vec::Vec;

use spin::Mutex;

struct Notice {
    supervisor: u32,
    pid: u32,
    code: i32,
}

static NOTICES: Mutex<Vec<Notice>> = Mutex::new(Vec::new());

pub(super) fn post(supervisor: u32, pid: u32, code: i32) {
    NOTICES.lock().push(Notice { supervisor, pid, code });
    crate::sched::wake_process(supervisor);
}

/// The next death notice for this supervisor, removed as it is taken.
pub(super) fn take(supervisor: u32) -> Option<(u32, i32)> {
    let mut notices = NOTICES.lock();
    let at = notices.iter().position(|n| n.supervisor == supervisor)?;
    let n = notices.remove(at);
    Some((n.pid, n.code))
}

/// Drop notices bound for a supervisor that is itself gone, so its pid, if
/// reused, does not collect a death meant for the process that had it before.
pub(super) fn forget_supervisor(supervisor: u32) {
    NOTICES.lock().retain(|n| n.supervisor != supervisor);
}
