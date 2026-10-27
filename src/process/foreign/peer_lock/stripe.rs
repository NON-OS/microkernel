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

//! Which lock a guest's address space is held under, and how a peer call
//! comes to hold the right one.
//!
//! One lock over every guest made each supervisor wait for every other's
//! copies and maps. A guest's own lock is enough: a call on one guest and
//! the teardown of another never touch the same tables. The locks are a
//! fixed set chosen by asid, so two guests can share one; that costs a
//! wait, never a missed exclusion.

/// How many locks the guests' address spaces are spread over.
pub(crate) const STRIPES: usize = 64;

/// The lock over the address space `asid` names.
pub(crate) fn stripe_of(asid: u32) -> usize {
    asid as usize % STRIPES
}

/// The asid `find` names, under its lock, named again once the lock is
/// held. The teardown frees an address space only under that same lock and
/// stops naming it there (peer_lock.rs), so one named again under the lock
/// is alive until the guard goes. None when `find` names none, before or
/// after the wait; one that names another after the wait is tried again.
pub(crate) fn hold<G>(
    find: impl Fn() -> Option<u32>,
    take: impl Fn(usize) -> G,
) -> Option<(u32, G)> {
    loop {
        let asid = find()?;
        let held = take(stripe_of(asid));
        match find() {
            Some(again) if again == asid => return Some((asid, held)),
            Some(_) => continue,
            None => return None,
        }
    }
}
