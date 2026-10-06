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

//! The serial line a driver's end gets from the kernel (`end_rule`):
//! `[EXIT] driver.e1000_0 status 2: no device present, not started`.

use super::end_rule::{told, words};
use crate::process::core::Pid;

pub(super) fn note(pid: Pid, status: i32, by_signal: bool) {
    let Some(pcb) = crate::process::core::PROCESS_TABLE.find_by_pid(pid) else {
        return;
    };
    // Never wait on the name: this runs on the exit path, fault handlers too.
    let Some(name) = pcb.name.try_lock() else {
        return;
    };
    if !told(&name, status, by_signal) {
        return;
    }
    let line = alloc::format!("[EXIT] {} status {}: {}", name.as_str(), status, words(status));
    drop(name);
    crate::sys::serial::println(line.as_bytes());
}
