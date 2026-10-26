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

//! The one table of kept exit statuses (kept_table.rs), filled by teardown
//! as it records the status in the reap log.

use spin::Mutex;

use super::kept_table::Kept;
use crate::process::core::Pid;

static KEPT: Mutex<Kept> = Mutex::new(Kept::new());

/// Keep `pid`'s status for a reader that is not its parent. Called before
/// the process can end, or its status is not kept.
pub(crate) fn keep_status_of(pid: Pid) -> bool {
    KEPT.lock().watch(pid)
}

pub(crate) fn kept_status(pid: Pid) -> Option<i32> {
    KEPT.lock().status(pid)
}

pub(super) fn note(pid: Pid, code: i32) {
    KEPT.lock().note(pid, code);
}
