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

//! The one way a process's argv is set: within ARGV_BYTES_MAX, or not at all.
//! An argv past the bound is said on serial and refused, and the process keeps
//! an empty argv, so nothing runs on a cut one (`argv_bound`).

use alloc::string::String;
use alloc::vec::Vec;

use super::argv_bound::{fits, joined_len, ARGV_BYTES_MAX};

/// Sets `pid`'s argv to `words`, or refuses them: false when they pass the
/// bound or the pid is gone.
pub fn set_argv(pid: u32, words: Vec<String>) -> bool {
    if !fits(&words) {
        let n = joined_len(&words);
        crate::log_warn!("[SPAWN] argv of {n} bytes passes {ARGV_BYTES_MAX}: refused");
        return false;
    }
    crate::process::with_process(pid, |pcb| *pcb.argv.lock() = words).is_some()
}
