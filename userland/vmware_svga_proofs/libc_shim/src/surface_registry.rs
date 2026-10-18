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

//! MkSurfaceShare (K20): each share names its receiver, and is kept, per
//! thread, so a test sees whom the driver named and how.

use std::cell::{Cell, RefCell};

/// The receiver may write its view.
pub const SURFACE_SHARE_WRITE: u64 = 1;

thread_local! {
    static SHARES: RefCell<Vec<(u64, u32, u64)>> = const { RefCell::new(Vec::new()) };
    static REFUSE: Cell<bool> = const { Cell::new(false) };
}

/// Every share made on this thread: the surface, the receiver, the flags.
pub fn shares() -> Vec<(u64, u32, u64)> {
    SHARES.with(|s| s.borrow().clone())
}

/// The next shares are refused, as a kernel refuses one past its receivers.
pub fn fail_share() {
    REFUSE.with(|r| r.set(true));
}

pub fn mk_surface_share(sid: u64, target_pid: u32, flags: u64) -> i64 {
    if REFUSE.with(|r| r.get()) || target_pid == 0 || flags & !SURFACE_SHARE_WRITE != 0 {
        return -22;
    }
    SHARES.with(|s| s.borrow_mut().push((sid, target_pid, flags)));
    0x5000 + sid as i64
}
