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

//! A sibling's orphan (L57). One owner thread's munmap lists the orphan of a
//! surface another process maps; a second thread munmapping the same range
//! keeps each page it pops of that orphan and hands it over, never frees it.

use alloc::vec::Vec;

use crate::kernel_core::surface_registry::types::SurfaceHandle;
use crate::memory::addr::PhysAddr;

use super::orphans::ORPHANS;

struct Own {
    handle: SurfaceHandle,
    popped: Vec<PhysAddr>,
}

/// An index entry for a frame two orphans list: held, credited to neither.
const SHARED: usize = usize::MAX;

pub(super) struct OwnOrphans {
    owner: u32,
    lists: Vec<Own>,
    /* Every frame of those orphans, sorted, with the list it is in. */
    index: Vec<(u64, usize)>,
}

impl OwnOrphans {
    /// Under the gate. An exit's orphans are held through its kept tables,
    /// and the frames of a DMA run are the broker's to free.
    pub(super) fn gather(owner: u32) -> Self {
        let (mut lists, mut index) = (Vec::new(), Vec::new());
        for o in ORPHANS.lock().iter() {
            if o.owner == owner && o.tables == 0 && o.grant.is_none() {
                index.extend(o.frames.iter().map(|f| (f.as_u64(), lists.len())));
                lists.push(Own { handle: o.handle, popped: Vec::new() });
            }
        }
        index.sort_unstable();
        // A frame two orphans list is kept and freed by neither (L61).
        index.dedup_by(|b, a| a.0 == b.0 && { a.1 = SHARED; true });
        Self { owner, lists, index }
    }

    /// True when the page-aligned `pa` is one of those orphans' frames.
    pub(super) fn holds(&mut self, pa: PhysAddr) -> bool {
        let Ok(at) = self.index.binary_search_by_key(&pa.as_u64(), |e| e.0) else { return false };
        if let Some(own) = self.lists.get_mut(self.index[at].1) {
            own.popped.push(pa);
        }
        true
    }

    /// Under the gate: what was popped joins the orphan's frames to free.
    pub(super) fn settle(self) {
        for o in self.lists.into_iter().filter(|o| !o.popped.is_empty()) {
            super::orphan_settle::settle_one(self.owner, o.handle, o.popped);
        }
    }
}
