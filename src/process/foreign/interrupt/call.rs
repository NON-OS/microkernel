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

//! The `MkForeignInterrupt` call: a supervisor marks one of its guests.

use crate::process::foreign::trap_table::is_parked;

/// Mark `pid`, one of the caller's guests. 1 says it is parked in a call
/// already, whose answer can carry the handler; 0 says it is marked.
pub fn sys_foreign_interrupt(pid: u64) -> i64 {
    let pid = match crate::process::foreign::signal_call::supervised(pid) {
        Ok(p) => p,
        Err(e) => return e,
    };
    if is_parked(pid) {
        return 1;
    }
    super::marks::mark(pid);
    0
}
