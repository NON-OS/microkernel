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

//! Device address space handed back below the top of a capsule's domain.
//! The bump pointer comes down only when the top grant goes, so a driver that
//! frees and maps buffers while an older grant stays live would creep toward
//! the 4 GiB limit and then fail every map. Pure, so the host proofs hold it.

extern crate alloc;

use alloc::vec::Vec;

use super::iova_space::after_give_back;

/// Free runs below the bump pointer as `(start, end)`, sorted, never empty
/// and never touching each other: neighbours are merged when a run returns.
/// None can touch the interrupt window, since no grant ever does.
pub(super) type Holes = Vec<(u64, u64)>;

/// The first hole `length` fits in, carved from its low end.
pub(super) fn take_hole(holes: &mut Holes, length: u64) -> Option<u64> {
    let i = holes.iter().position(|&(start, end)| length > 0 && end - start >= length)?;
    let start = holes[i].0;
    if holes[i].1 - start == length {
        holes.remove(i);
    } else {
        holes[i].0 = start + length;
    }
    Some(start)
}

/// Take `[iova, iova + length)` back. The top run lowers the bump pointer,
/// which then swallows every hole that ends where it now stands; any other
/// run becomes a hole, merged with the holes on either side of it.
pub(super) fn give_back(next: &mut u64, holes: &mut Holes, iova: u64, length: u64) {
    let Some(end) = iova.checked_add(length).filter(|_| length > 0) else {
        return;
    };
    let lowered = after_give_back(*next, iova, length);
    if lowered != *next {
        *next = lowered;
        while let Some(&(start, top)) = holes.last() {
            if top != *next {
                break;
            }
            *next = start;
            holes.pop();
        }
        return;
    }
    let i = holes.partition_point(|&(start, _)| start < iova);
    holes.insert(i, (iova, end));
    if holes.get(i + 1).is_some_and(|&(start, _)| start == end) {
        holes[i].1 = holes.remove(i + 1).1;
    }
    if i > 0 && holes[i - 1].1 == iova {
        holes[i - 1].1 = holes.remove(i).1;
    }
}
