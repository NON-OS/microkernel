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
//!
//! Knowing a pid buys nothing: the target must be foreign and must name
//! the caller as its supervisor. Both conditions are set once at creation
//! and never move.

use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_PERM};

pub(super) const PAGE: u64 = 4096;

/*
 * One call maps or copies at most this much, so a guest image crosses in
 * bounded pieces and no single call holds the processor.
 */
pub(super) const MAX_SPAN: u64 = 1 << 20;

pub const PROT_WRITE: u64 = 1 << 0;
pub const PROT_EXEC: u64 = 1 << 1;

/// The guest's address space, or the errno the caller gets instead.
pub(super) fn supervised_asid(caller: u32, pid: u32) -> Result<u32, i64> {
    if super::registry::supervisor_of(pid) != Some(caller) {
        return Err(ERRNO_PERM);
    }
    crate::memory::paging::manager::lookup_asid_for_process(pid).ok_or(ERRNO_INVAL)
}
