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

use core::sync::atomic::{AtomicU32, Ordering};

static VSYNC_TRACE_COUNT: AtomicU32 = AtomicU32::new(0);

pub(super) fn trace_surface(op: &[u8], label: &[u8], pid: u32) {
    if !matches!(pid, 0x17 | 0x26 | 0x27) || VSYNC_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) >= 80
    {
        return;
    }
    crate::sys::serial::trace(b"[SURFACE] ");
    crate::sys::serial::trace(op);
    crate::sys::serial::trace(b" ");
    crate::sys::serial::traceln(label);
}
