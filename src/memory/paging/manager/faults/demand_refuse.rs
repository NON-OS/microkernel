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

//! The pages the kernel never fills on a fault.

use crate::memory::layout;
use crate::memory::paging::constants::PAGE_SIZE_4K;

/// True when a not-present fault at `addr` in `pid` must not be filled.
pub(super) fn refused(addr: u64, pid: u32) -> bool {
    /*
     * Only user-space addresses may be demand-backed. A not-present fault
     * in the kernel half is never a legitimate lazy mapping; backing it
     * silently would hand a capsule kernel-range memory. Surface it as an
     * unhandled fault so the fault path kills the offender (user) or traps
     * the real kernel bug, instead of papering over it.
     */
    if !layout::in_user_space(addr) {
        return true;
    }
    /*
     * Never demand-back the null page. A fault in the lowest page is a null
     * or near-null dereference; backing it would silently satisfy the bug
     * instead of trapping it. Leave the page unmapped as a guard so the
     * fault path kills the offending capsule.
     */
    if addr < PAGE_SIZE_4K as u64 {
        return true;
    }
    /*
     * A foreign guest's pages are exactly the ones its supervisor mapped for
     * it. Filling any other page would hand the guest memory nobody gave it:
     * a PROT_NONE reservation, a guard page, a hole. So the fault is refused,
     * the fault path ends the thread, and its supervisor is told and decides
     * what that means for the guest.
     */
    crate::process::foreign::is_foreign(pid)
}
