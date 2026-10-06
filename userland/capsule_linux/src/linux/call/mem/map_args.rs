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

//! What Linux refuses of an mmap before it looks for room, from the
//! arguments alone, and where a hint asks the mapping to go. Pure, so the
//! host proofs hold every refusal and its errno.

use crate::linux::abi::errno;
use crate::linux::guest::{page_down, page_len, PAGE};

use super::prot_args::wx_refused;

pub const MAP_SHARED: u64 = 0x01;
pub const MAP_PRIVATE: u64 = 0x02;
/// MAP_SHARED with every flag checked: shared, as far as this capsule goes.
const MAP_SHARED_VALIDATE: u64 = 0x03;
/// The bits that say shared or private.
const MAP_TYPE: u64 = 0x0f;
pub const MAP_FIXED: u64 = 0x10;
pub const MAP_FIXED_NOREPLACE: u64 = 0x10_0000;
/// Linux's default vm.mmap_min_addr. Nothing is mapped at an exact address
/// below it, so a null pointer, or one a little past null, is never memory.
pub const MIN_ADDR: u64 = 0x1_0000;

/// True for MAP_FIXED and MAP_FIXED_NOREPLACE: the exact address or failure.
pub fn exact(flags: u64) -> bool {
    flags & (MAP_FIXED | MAP_FIXED_NOREPLACE) != 0
}

/// Linux's order: a misaligned offset or an empty length EINVAL, neither
/// shared nor private EINVAL, an exact address off a page boundary EINVAL, a
/// length that rounds past the top ENOMEM. Then this personality's own: a
/// page both writable and executable, or an exact address below
/// mmap_min_addr, EPERM.
pub fn map_args(addr: u64, len: u64, prot: u64, flags: u64, off: u64) -> Result<(), i64> {
    if !off.is_multiple_of(PAGE) || len == 0 {
        return Err(errno::EINVAL);
    }
    if !matches!(flags & MAP_TYPE, MAP_SHARED | MAP_PRIVATE | MAP_SHARED_VALIDATE) {
        return Err(errno::EINVAL);
    }
    if exact(flags) && !addr.is_multiple_of(PAGE) {
        return Err(errno::EINVAL);
    }
    if page_len(len).is_none() {
        return Err(errno::ENOMEM);
    }
    if wx_refused(prot) {
        return Err(errno::EPERM);
    }
    if exact(flags) && addr < MIN_ADDR {
        return Err(errno::EPERM);
    }
    Ok(())
}

/// Where a hint asks the mapping to go, None for no hint. Linux rounds a
/// hint down to its page, so one inside page zero is no hint at all, and
/// moves one below mmap_min_addr up to it.
pub fn hint(addr: u64) -> Option<u64> {
    match page_down(addr) {
        0 => None,
        at if at < MIN_ADDR => Some(MIN_ADDR),
        at => Some(at),
    }
}
