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

//! Taking the next request. A recovery's bound (nvm::bound) is for requests
//! already queued while it ran; one sent after the inbox was seen empty, or
//! after the first other sender it served sent again (nvm::served), has its
//! own time. libc has no receive that returns at once, so while a bound
//! is pending the inbox is looked at with a 1 ms timeout first: the kernel
//! hands a queued message back at once and times out only when none came
//! (src/syscall/microkernel/ipc/recv_from.rs), and then the bound is
//! dropped. A request that lands within that millisecond is taken as
//! queued, and refused only if the bound has passed.

use crate::nvm::IoQueue;

/// The kernel's answer to a receive whose timeout ran out with nothing, as
/// nonos_libc's bringup::policy names it.
pub const ERRNO_TIMEDOUT: i64 = -110;

/// Receive the next request with `recv(timeout_ms)`, 0 waiting for one.
pub fn take<R: FnMut(u64) -> i64>(io: Option<&mut IoQueue>, mut recv: R) -> i64 {
    if let Some(io) = io.filter(|io| io.bound_pending()) {
        let n = recv(1);
        if n != ERRNO_TIMEDOUT {
            return n;
        }
        io.inbox_empty();
    }
    recv(0)
}
