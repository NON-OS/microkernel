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

//! The process calls: the host's allocator is already up, an exit is a proof
//! failure, and a process ends only when the proof ends it.

use std::sync::Mutex;

pub fn heap_init() -> Result<(), ()> {
    Ok(())
}

pub fn mk_exit(status: i32) -> ! {
    panic!("capsule exited with status {status}");
}

pub fn mk_debug(_buf: *const u8, len: usize) -> i64 {
    len as i64
}

static ENDED: Mutex<Vec<u32>> = Mutex::new(Vec::new());

/// Every process runs until a proof says it ended.
pub fn mk_pid_alive(pid: u32) -> bool {
    !ENDED.lock().unwrap_or_else(|e| e.into_inner()).contains(&pid)
}

/// Have `pid` read as ended from now on.
pub fn end_pid(pid: u32) {
    ENDED.lock().unwrap_or_else(|e| e.into_inner()).push(pid);
}

/// Every process runs again, for the next proof.
pub fn revive_all() {
    ENDED.lock().unwrap_or_else(|e| e.into_inner()).clear();
}
