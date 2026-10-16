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
//! finds no slot frames to claim and, as the owner, no foreign window. Each
//! page it pops of the orphan is kept and handed to it, never freed.

use alloc::vec::Vec;

use crate::kernel_core::surface_registry::types::SurfaceHandle;
use crate::memory::addr::PhysAddr;

use super::orphans::ORPHANS;

struct Own {
    handle: SurfaceHandle,
    frames: Vec<PhysAddr>,
    popped: Vec<PhysAddr>,
}

pub(super) struct OwnOrphans {
    owner: u32,
    lists: Vec<Own>,
    /* Every frame of those orphans, sorted, with the list it is in. */
    index: Vec<(u64, usize)>,
}

impl OwnOrphans {
    /// Under the gate. An exit's orphans are held through its kept tables,
    /// and the frames of a DMA run are the broker's to free.
    pub(super) fn collect(owner: u32) -> Self {
        let lists: Vec<Own> = ORPHANS
            .lock()
            .iter()
            .filter(|o| o.owner == owner && o.tables == 0 && o.grant.is_none())
            .map(|o| Own { handle: o.handle, frames: o.frames.clone(), popped: Vec::new() })
            .collect();
        let mut index: Vec<(u64, usize)> = Vec::new();
        for (i, o) in lists.iter().enumerate() {
            index.extend(o.frames.iter().map(|f| (f.as_u64(), i)));
        }
        index.sort_unstable();
        Self { owner, lists, index }
    }

    /// True when the page-aligned `pa` is one of those orphans' frames.
    pub(super) fn holds(&mut self, pa: PhysAddr) -> bool {
        let found = self.index.binary_search_by_key(&pa.as_u64(), |&(f, _)| f);
        let Ok(at) = found else { return false };
        self.lists[self.index[at].1].popped.push(pa);
        true
    }

    /// Under the gate: what was popped joins the orphan's frames to free.
    pub(super) fn settle(self) {
        for o in self.lists.into_iter().filter(|o| !o.popped.is_empty()) {
            super::orphan_settle::settle_one(self.owner, o.handle, o.frames, o.popped);
        }
    }
}
