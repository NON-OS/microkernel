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
//! Whether a message came from the window manager.
//!
//! Any process that may use IPC may send any pid a frame shaped like the
//! WM's notice. The kernel records the true sender, so the WM's pid is what
//! separates its word on this window's focus from a forged one.

use core::sync::atomic::{AtomicU32, Ordering};

use super::lookup_service::lookup_service;

static WM_PID: AtomicU32 = AtomicU32::new(0);

/// True only when `sender` is the pid that owns the `wm` service. A WM
/// that restarted is looked up again before a message is refused.
pub fn from_wm(sender: u32) -> bool {
    let known = WM_PID.load(Ordering::Acquire);
    if known != 0 && known == sender {
        return true;
    }
    let Some(wm) = lookup_service(b"wm") else {
        return false;
    };
    WM_PID.store(wm.pid, Ordering::Release);
    wm.pid == sender
}
