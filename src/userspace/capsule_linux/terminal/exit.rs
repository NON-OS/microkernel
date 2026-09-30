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
//! A terminal run ends with the terminal that started it, and its slot is
//! free again once it is gone.

use crate::process::signal::SIGKILL;

/// `parent` is exiting: end every run it started. A run outliving its
/// terminal would hold a slot, and a model's memory, for no one to read.
pub fn end_terminal_runs_of(parent: u32) {
    for pid in super::held::held_by(parent) {
        crate::process::exit::teardown(pid, 128 + SIGKILL as i32, true);
    }
}

/// `pid` is gone and its endpoints are unregistered: free its slot, if it
/// held one.
pub fn terminal_run_gone(pid: u32) {
    super::held::release(pid);
}
