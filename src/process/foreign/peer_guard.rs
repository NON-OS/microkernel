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

//! The one check every peer call makes before it touches a guest.

use super::peer_lock::Held;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_PERM};

pub(super) const PAGE: u64 = 4096;

/*
 * One call maps or copies at most this much, so a guest image crosses in
 * bounded pieces and no single call holds the processor.
 */
pub(super) const MAX_SPAN: u64 = 1 << 20;

/* The first address of the kernel half. */
pub(super) const USER_VA_END: u64 = 0x0000_8000_0000_0000;

/// True when `[addr, addr + len)` lies wholly in the guest's own half.
pub(super) fn in_user_half(addr: u64, len: u64) -> bool {
    match addr.checked_add(len) {
        Some(end) => end <= USER_VA_END,
        None => false,
    }
}

pub(super) const PROT_WRITE: u64 = 1 << 0;
pub(super) const PROT_EXEC: u64 = 1 << 1;
/*
 * No access from the guest at all. The page stays present with the user bit
 * clear, so every guest access faults and the frame keeps its bytes for a
 * later protection that allows access, as Linux keeps them.
 */
pub(super) const PROT_NONE: u64 = 1 << 2;


/// The pid a syscall argument names. Refused rather than truncated: `as u32`
/// on 2^32 + n would name process n, a real one the caller never asked for.
pub(super) fn pid_arg(raw: u64) -> Result<u32, i64> {
    u32::try_from(raw).map_err(|_| ERRNO_INVAL)
}

/// The guest's address space and the lock over it, or the errno the
/// caller gets instead.
///
/// The lock is taken before the guest is looked up, so the check and every
/// use of the address space after it are one step: a guest's teardown
/// drops its row first and frees its tables only under this lock
/// (`without_peer_calls`), so a call that finds the row finds the tables
/// alive until it returns.
pub(super) fn supervised_asid(caller: u32, pid: u64) -> Result<(u32, Held), i64> {
    let pid = pid_arg(pid)?;
    let held = super::peer_lock::take();
    if super::registry::supervisor_of(pid) != Some(caller) {
        return Err(ERRNO_PERM);
    }
    let asid = crate::memory::paging::manager::lookup_asid_for_process(pid).ok_or(ERRNO_INVAL)?;
    Ok((asid, held))
}
