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

//! The arguments of mremap and munmap, checked as Linux checks them before
//! it looks at a mapping, and the end of a span checked against the top of
//! the guest's area. Pure, so the host proofs hold every refusal.

use crate::linux::abi::errno;
use crate::linux::guest::{page_len, PAGE, USER_MAX};

pub const MREMAP_MAYMOVE: u64 = 1;
const MREMAP_FIXED: u64 = 2;
const MREMAP_DONTUNMAP: u64 = 4;

/// The old and new lengths in whole pages, or EINVAL for what Linux refuses
/// with it: an unknown flag, FIXED without MAYMOVE, DONTUNMAP without
/// MAYMOVE or with a change of size, a misaligned address, a new length that
/// is zero or rounds past the top. FIXED and DONTUNMAP name a destination,
/// which this capsule does not offer, and an old length of zero asks for a
/// second view of a shared mapping, which no mapping here is: both EINVAL,
/// never a success that did something else.
pub fn remap_args(old: u64, old_len: u64, new_len: u64, flags: u64) -> Result<(u64, u64), i64> {
    if flags & !(MREMAP_MAYMOVE | MREMAP_FIXED | MREMAP_DONTUNMAP) != 0 {
        return Err(errno::EINVAL);
    }
    let moves = flags & MREMAP_MAYMOVE != 0;
    if flags & MREMAP_FIXED != 0 && !moves {
        return Err(errno::EINVAL);
    }
    if flags & MREMAP_DONTUNMAP != 0 && (!moves || old_len != new_len) {
        return Err(errno::EINVAL);
    }
    if !old.is_multiple_of(PAGE) {
        return Err(errno::EINVAL);
    }
    let (Some(old_len), Some(new_len)) = (page_len(old_len), page_len(new_len)) else {
        return Err(errno::EINVAL);
    };
    if new_len == 0 || old_len == 0 || flags & (MREMAP_FIXED | MREMAP_DONTUNMAP) != 0 {
        return Err(errno::EINVAL);
    }
    Ok((old_len, new_len))
}

/// The end of `[at, at + len)`, or None when it wraps or passes the top of
/// the guest's area. A span a guest names is checked through this before
/// its end is used for anything.
pub fn end_of(at: u64, len: u64) -> Option<u64> {
    at.checked_add(len).filter(|&end| end <= USER_MAX)
}

/// munmap's span: an address on a page boundary and a length that is not
/// zero, ending inside the guest's area once rounded up to a page. Linux
/// answers every other case EINVAL.
pub fn unmap_span(addr: u64, len: u64) -> Result<(u64, u64), i64> {
    if !addr.is_multiple_of(PAGE) {
        return Err(errno::EINVAL);
    }
    let end = page_len(len).and_then(|l| end_of(addr, l)).filter(|&e| e > addr);
    end.map(|e| (addr, e - addr)).ok_or(errno::EINVAL)
}
