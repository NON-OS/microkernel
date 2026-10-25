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

//! MkSurfaceShare as K20 has it: each share names the process it is for,
//! recorded per thread so a test reads who was named and with what flags.

use std::cell::RefCell;

use super::surface::SHARED_HANDLE_BASE;

/// The receiver may write its view (MkSurfaceShare flags).
pub const SURFACE_SHARE_WRITE: u64 = 1;

/// One share: the surface, the process it names and the flags it gives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Share {
    pub sid: u64,
    pub target_pid: u32,
    pub flags: u64,
}

thread_local! {
    static SHARES: RefCell<Vec<Share>> = const { RefCell::new(Vec::new()) };
    static RELEASED: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
}

/// Every handle given back on this thread, in order.
pub fn released() -> Vec<u64> {
    RELEASED.with(|r| r.borrow().clone())
}

/// MkSurfaceRelease: the handle is recorded, and given back in order with the
/// unmaps and the claim's release; the kernel's 0 answered.
pub fn mk_surface_release(handle: u64) -> i64 {
    RELEASED.with(|r| r.borrow_mut().push(handle));
    crate::claim::give_back("surface_release", handle)
}

/// Every share made on this thread, in order.
pub fn shares() -> Vec<Share> {
    SHARES.with(|s| s.borrow().clone())
}

/// The surface's handle, as the kernel answers a share naming a receiver;
/// a flag bit no flag names is EINVAL, as there.
pub fn mk_surface_share(sid: u64, target_pid: u32, flags: u64) -> i64 {
    if flags & !SURFACE_SHARE_WRITE != 0 {
        return -22;
    }
    SHARES.with(|s| s.borrow_mut().push(Share { sid, target_pid, flags }));
    SHARED_HANDLE_BASE + sid as i64
}
