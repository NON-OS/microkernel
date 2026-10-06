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

//! The serial trace of sends from a few fixed pids, capped at 48 lines.

use core::sync::atomic::{AtomicU32, Ordering};

static SEND_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

fn is_traced(pid: u32) -> bool {
    matches!(pid, 0x18 | 0x1a | 0x1b)
}

pub(super) fn trace(pid: u32, endpoint: u64, target: &str, len: usize) {
    if !is_traced(pid) || SEND_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 48 {
        return;
    }
    crate::sys::serial::trace(b"[IPC-SEND] pid=");
    crate::sys::serial::trace_hex(pid as u64);
    crate::sys::serial::trace(b" ep=");
    crate::sys::serial::trace_hex(endpoint);
    crate::sys::serial::trace(b" len=");
    crate::sys::serial::trace_dec(len as u64);
    crate::sys::serial::trace(b" target=");
    crate::sys::serial::trace(target.as_bytes());
    crate::sys::serial::traceln(b"");
}
