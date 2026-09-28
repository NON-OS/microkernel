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
use crate::linux::guest::{page_up, span_within, Guest, Region, PAGE, USER_MAX};

const MAYMOVE: u64 = 1;

pub fn mremap(guest: &mut Guest, old: u64, old_len: u64, new_len: u64, flags: u64) -> u64 {
    // MREMAP_FIXED and DONTUNMAP choose the destination; neither is offered.
    if flags & !MAYMOVE != 0 || old % PAGE != 0 || old_len == 0 || new_len == 0 {
        return errno::fail(errno::EINVAL);
    }
    let (old_len, new_len) = (page_up(old_len), page_up(new_len));
    let Some(r) = guest.regions.iter().find(|r| r.at <= old && old < r.at + r.len).copied() else {
        return errno::fail(errno::EFAULT);
    };
    // Linux moves one mapping at a time: the old span must lie inside one.
    if !one_mapping(guest, old, old_len, &r) {
        return errno::fail(errno::EFAULT);
    }
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
    // The grown part is the same mapping: its protection, its backing.
    if !guest.overlaps(tail, grow)
        && span_within(tail, grow, USER_MAX).is_some()
        && guest.map_like(tail, grow, &r) >= 0
    {
        return errno::ok(old);
    }
    if flags & MAYMOVE == 0 {
        return errno::fail(errno::ENOMEM);
    }
    super::remap_move::moved(guest, old, old_len, new_len, &r)
}

/// Every page of `[at, at + len)` is held with the same protection, backing
/// and provenance as `like`, which is what Linux keeps as one mapping.
fn one_mapping(guest: &Guest, at: u64, len: u64, like: &Region) -> bool {
    let end = at + len;
    let mut reach = at;
    while reach < end {
        let Some(r) = guest.regions.iter().find(|r| r.at <= reach && reach < r.at + r.len) else {
            return false;
        };
        let same = (r.write, r.exec, r.access, r.backed, r.unproven)
            == (like.write, like.exec, like.access, like.backed, like.unproven);
        if !same {
            return false;
        }
        reach = r.at + r.len;
    }
    true
}
