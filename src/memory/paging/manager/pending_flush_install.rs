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

//! What installing a leaf owes, which is where the manager skips a remote
//! flush.

use super::pending_flush::PendingFlush;
use crate::memory::addr::VirtAddr;
use crate::memory::paging::tlb;

impl PendingFlush {
    /// What installing a leaf at `va` owes, given whether the entry it
    /// overwrote was present.
    ///
    /// A present entry may be cached by any cpu running `asid`, so replacing
    /// it (another frame, fewer rights, or even more rights, which a peer
    /// holding the old entry would fault on with nothing to repair it) owes a
    /// flush everywhere. An absent one owes nothing anywhere: x86 caches a
    /// translation, in the TLB or in the paging-structure caches, only from
    /// entries whose present bit is set, so no cpu can hold anything for a leaf
    /// that was not present, nor for the intermediate tables `install` has just
    /// created under entries that were not present either. Intermediate
    /// entries that were already present are not changed by an install, so
    /// what a peer caches of them stays correct. This is the architected rule
    /// (Intel SDM vol. 3A, 4.10.4.3: changing P from 0 to 1 needs no
    /// invalidation; AMD caches only valid translations likewise), and it
    /// does not depend on PCIDs.
    ///
    /// That holds only if an absent entry really is uncached, which is the
    /// release order every unmap in this manager keeps: the leaf is cleared,
    /// the flush is committed and acknowledged, and only then are the frame
    /// and the virtual range given back. An address is therefore never handed
    /// out again while some cpu still caches its previous translation. Page
    /// tables themselves are freed only with an address space no cpu is
    /// running, and loading CR3 drops the paging-structure caches for it.
    ///
    /// The local `invlpg` is not required by that argument; it is kept because
    /// it costs one instruction on this cpu and no other, and it keeps this
    /// cpu clean even if some path ever clears an entry without flushing.
    pub(super) fn after_install(va: VirtAddr, asid: u32, replaced_present: bool) -> Self {
        if replaced_present {
            return Self::one_local_now(va, asid);
        }
        tlb::invalidate_page(va);
        Self::none()
    }
}
