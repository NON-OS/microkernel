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

//! One line now and then saying how a receive op has fared: its calls, how
//! many went unanswered in time, and the bound it has learned. Sparse, at
//! 256, 1024, 4096 and 16384 calls, so a boot log shows a slow driver being
//! waited for without one line per poll.

extern crate alloc;

use nonos_libc::mk_debug;

/// Whether the `calls`th call of an op is one the line is said at.
pub const fn due(calls: u64) -> bool {
    matches!(calls, 256 | 1024 | 4096 | 16384)
}

/// Say the line for an op, when its call count is due.
pub fn at(batch: bool, calls: u64, timeouts: u64, bound_ms: u64) {
    if !due(calls) {
        return;
    }
    let op = if batch { "batch" } else { "single" };
    let line = alloc::format!(
        "[NET-CORE] rx {op}: {calls} calls, {timeouts} not answered in time, bound {bound_ms} ms\n"
    );
    let _ = mk_debug(line.as_ptr(), line.len());
}
