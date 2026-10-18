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

//! The preempt path's trace lines, kept to the pids a past hang was chased
//! through and to the first forty, so a busy machine's log stays readable.

use core::sync::atomic::{AtomicU32, Ordering};

static PREEMPT_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

pub(super) fn trace(label: &[u8], pid: u32) {
    if !matches!(pid, 0x1b | 0x1c | 0x26 | 0x27)
        || PREEMPT_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 40
    {
        return;
    }
    crate::sys::serial::print(b"[PREEMPT] ");
    crate::sys::serial::println(label);
}
