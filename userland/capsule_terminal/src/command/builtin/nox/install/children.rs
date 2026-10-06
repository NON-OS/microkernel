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

//! The terminal's own children, as the process table reports them. The
//! installer makes the terminal the parent of what it loads for it, so a child
//! that was not there before a load, and is no older than the load, is what
//! it loaded. Only the pid, parent and age of each entry are read, which the
//! table shows any caller. The desktop finds a Launchpad app the same way.

use alloc::vec::Vec;

use nonos_libc::mk_getpid;

use crate::command::builtin::proc_table::table;

/// Slack on the age test for the time between the load and the first look.
const AGE_SLACK_MS: u64 = 1_000;

/// The pids of the terminal's children now.
pub(super) fn children() -> Vec<u32> {
    let me = mk_getpid();
    table().iter().filter(|e| e.ppid == me && e.pid != 0).map(|e| e.pid).collect()
}

/// A child born since `before` was taken and within `age_ms`, the youngest
/// if there are several.
pub(super) fn new_child(before: &[u32], age_ms: i64) -> Option<u32> {
    let me = mk_getpid();
    let limit = (age_ms.max(0) as u64).saturating_add(AGE_SLACK_MS);
    table()
        .iter()
        .filter(|e| e.ppid == me && e.pid != 0 && !before.contains(&e.pid))
        .filter(|e| e.uptime_ms <= limit)
        .min_by_key(|e| e.uptime_ms)
        .map(|e| e.pid)
}
