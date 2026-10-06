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

//! `mremap`: shrink in place, grow in place when the pages after are free,
//! or move with MREMAP_MAYMOVE. glibc's realloc of a large block is this.
//! The arguments are checked in `remap_args`, which the host proofs hold.

use crate::linux::abi::errno;
use crate::linux::guest::{span_within, Guest, USER_MAX};

use super::remap_args::{end_of, remap_args, MREMAP_MAYMOVE};

pub fn mremap(guest: &mut Guest, old: u64, old_len: u64, new_len: u64, flags: u64) -> u64 {
    let (old_len, new_len) = match remap_args(old, old_len, new_len, flags) {
        Ok(lens) => lens,
        Err(e) => return errno::fail(e),
    };
    /*
     * Every end is checked, since both lengths are the guest's: an old span
     * that wraps would pass the one-mapping walk below having walked nothing.
     */
    let Some(old_end) = end_of(old, old_len) else {
        return errno::fail(if new_len <= old_len { errno::EINVAL } else { errno::EFAULT });
    };
    /* Shrinking is unmapping the tail, which Linux does before any lookup. */
    if new_len <= old_len {
        if new_len < old_len && guest.unmap(old + new_len, old_len - new_len) < 0 {
            return errno::fail(errno::EINVAL);
        }
        return errno::ok(old);
    }
    let Some(r) = guest.regions.iter().find(|r| r.at <= old && old < r.at + r.len).copied() else {
        return errno::fail(errno::EFAULT);
    };
    /* Linux moves one mapping at a time: the old span must lie inside one. */
    if !super::remap_one::one_mapping(guest, old, old_len, &r) {
        return errno::fail(errno::EFAULT);
    }
    /* Code was proved where it was mapped; a moved copy would not be. */
    if r.exec {
        return errno::fail(errno::EPERM);
    }
    let grow = new_len - old_len;
    /* The grown part is the same mapping: its protection, its backing. */
    if !guest.overlaps(old_end, grow)
        && span_within(old_end, grow, USER_MAX).is_some()
        && guest.map_like(old_end, grow, &r) >= 0
    {
        guest.mmap_next = guest.mmap_next.max(old_end + grow);
        if guest.span_kept(old, old_len) {
            guest.mark_kept(old_end, grow);
        }
        return errno::ok(old);
    }
    if flags & MREMAP_MAYMOVE == 0 {
        return errno::fail(errno::ENOMEM);
    }
    super::remap_move::moved(guest, old, old_len, new_len, &r)
}
