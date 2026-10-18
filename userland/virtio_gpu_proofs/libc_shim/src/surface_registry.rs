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

//! MkSurfaceShare: each share the included handler makes is kept for
//! the test to read, the receiver it names and its flags, as the kernel
//! would record them, and answered with a handle for the surface.

use std::cell::RefCell;

/// The receiver may write its view (MkSurfaceShare flags).
pub const SURFACE_SHARE_WRITE: u64 = 1;
/// What a share answers for the surface registered as `sid`.
pub const SHARED_HANDLE_BASE: i64 = 0x7700;

thread_local! {
    static SHARES: RefCell<Vec<(u64, u32, u64)>> = const { RefCell::new(Vec::new()) };
}

/// A flag bit no flag names is EINVAL, as the kernel's.
pub fn mk_surface_share(sid: u64, target_pid: u32, flags: u64) -> i64 {
    if flags & !SURFACE_SHARE_WRITE != 0 {
        return -22;
    }
    SHARES.with(|s| s.borrow_mut().push((sid, target_pid, flags)));
    SHARED_HANDLE_BASE + sid as i64
}

/// Every share on this thread, in order: the surface, its receiver, flags.
pub fn shares() -> Vec<(u64, u32, u64)> {
    SHARES.with(|s| s.borrow().clone())
}
