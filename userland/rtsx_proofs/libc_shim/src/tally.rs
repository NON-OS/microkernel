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

//! What a device access that never reached memory was: stopped because bus
//! mastering was off, or a violation, inside no live grant.

use std::cell::Cell;

use crate::events::note;

thread_local! {
    static VIOLATIONS: Cell<u32> = const { Cell::new(0) };
    static BLOCKED: Cell<u32> = const { Cell::new(0) };
}

pub(crate) fn count_blocked(iova: u64, len: usize) {
    BLOCKED.with(|b| b.set(b.get() + 1));
    note(format!("dma blocked {iova:#x}+{len}"));
}

pub(crate) fn count_violation(iova: u64, len: usize) {
    VIOLATIONS.with(|v| v.set(v.get() + 1));
    note(format!("dma violation {iova:#x}+{len}"));
}

/// Device accesses outside every live grant on this thread.
pub fn violations() -> u32 {
    VIOLATIONS.with(|v| v.get())
}

/// Device accesses stopped by bus mastering being off, on this thread.
pub fn blocked() -> u32 {
    BLOCKED.with(|b| b.get())
}
