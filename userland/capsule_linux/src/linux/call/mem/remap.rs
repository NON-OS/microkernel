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

use crate::linux::abi::errno;
use crate::linux::guest::{page_up, span_within, Guest, MMAP_LIMIT, PAGE};

const MAYMOVE: u64 = 1;

pub fn mremap(guest: &mut Guest, old: u64, old_len: u64, new_len: u64, flags: u64) -> u64 {
    // MREMAP_FIXED and DONTUNMAP choose the destination; neither is offered.
    if flags & !MAYMOVE != 0 || old % PAGE != 0 || old_len == 0 || new_len == 0 {
        return errno::fail(errno::EINVAL);
    }
    let (old_len, new_len) = (page_up(old_len), page_up(new_len));
    if guest.mapped_from(old) < old_len {
        return errno::fail(errno::EFAULT);
    }
    let Some(r) = guest.regions.iter().find(|r| r.at <= old && old < r.at + r.len).copied() else {
        return errno::fail(errno::EFAULT);
    };
    // Code was proved where it was mapped; a moved copy would not be.
    if r.exec {
        return errno::fail(errno::EPERM);
    }
    if new_len <= old_len {
        if new_len < old_len && guest.unmap(old + new_len, old_len - new_len) < 0 {
            return errno::fail(errno::EINVAL);
        }
        return errno::ok(old);
    }
    let tail = old + old_len;
    let grow = new_len - old_len;
    let free = !guest.regions.iter().any(|g| g.at < tail + grow && tail < g.at + g.len);
    if free
        && span_within(tail, grow, MMAP_LIMIT).is_some()
        && guest.map(tail, grow, r.write, false) >= 0
    {
        guest.mmap_next = guest.mmap_next.max(tail + grow);
        if guest.span_kept(old, old_len) {
            guest.mark_kept(tail, grow);
        }
        return errno::ok(old);
    }
    if flags & MAYMOVE == 0 {
        return errno::fail(errno::ENOMEM);
    }
    super::remap_move::moved(guest, old, old_len, new_len, r.write)
}
