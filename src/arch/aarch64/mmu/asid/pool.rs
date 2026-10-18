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

//! Which hardware ASID each user root is tagged with, so a switch needs no
//! TLB flush: entries for many roots live side by side, told apart by ASID
//! (Arm ARM, DDI 0487, ASIDs). Pure, so `asid_proofs` holds it to its rules.

use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;

/// What a root is tagged with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Tag {
    /// Its own ASID, held until the root is retired.
    Own(u16),
    /// All taken: the shared one, dropped on each load of such a root.
    Shared(u16),
}

/// ASID 0 is the kernel's, the root boot runs on; the top one is shared.
/// Between them, one per live user root.
pub(crate) struct Pool {
    shared: u16,
    used: Vec<u64>,
    by_root: BTreeMap<u64, u16>,
}

impl Pool {
    /// `bits`: the ASID width TCR_EL1 selects, 8 or 16.
    pub(crate) fn new(bits: u32) -> Self {
        let count = 1usize << bits.min(16);
        let shared = (count - 1) as u16;
        Pool { shared, used: vec![0; count.div_ceil(64)], by_root: BTreeMap::new() }
    }

    pub(crate) fn tag(&mut self, root: u64) -> Tag {
        if let Some(&asid) = self.by_root.get(&root) {
            return Tag::Own(asid);
        }
        match (1..self.shared).find(|&a| !self.is_used(a)) {
            Some(asid) => {
                self.used[usize::from(asid) / 64] |= 1 << (asid % 64);
                self.by_root.insert(root, asid);
                Tag::Own(asid)
            }
            None => Tag::Shared(self.shared),
        }
    }

    /// The ASID `root` held, now free. The caller invalidates it before
    /// anything can be handed it again, so it must hold the pool across both.
    pub(crate) fn retire(&mut self, root: u64) -> Option<u16> {
        let asid = self.by_root.remove(&root)?;
        self.used[usize::from(asid) / 64] &= !(1 << (asid % 64));
        Some(asid)
    }

    fn is_used(&self, asid: u16) -> bool {
        self.used[usize::from(asid) / 64] & (1 << (asid % 64)) != 0
    }
}
