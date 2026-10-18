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

//! Processes, as far as the server asks after them: every pid is alive
//! until a test says it exited.

use std::sync::Mutex;

static EXITED: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn exited() -> std::sync::MutexGuard<'static, Vec<u32>> {
    EXITED.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// `pid` has exited.
pub fn pid_exits(pid: u32) {
    exited().push(pid);
}

pub extern "C" fn mk_pid_alive(pid: u32) -> bool {
    !exited().contains(&pid)
}
