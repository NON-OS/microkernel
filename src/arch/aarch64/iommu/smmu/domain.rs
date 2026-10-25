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

//! A translation domain: its stage 1 tables, its CD and the streams that
//! translate through it.

use crate::memory::iommu::DomainId;

/// Domains the driver holds at once. Ids 1 to 64 name them; id 0 is the
/// kernel's and is never handed out.
pub(super) const MAX_DOMAINS: usize = 64;
/// Streams one domain holds; a capsule claims a handful of devices.
pub(super) const MAX_STREAMS: usize = 16;

#[derive(Debug, Clone, Copy)]
pub(super) struct Domain {
    pub(super) ttb: u64, // its level 1 table
    /// The frame holding the CD every one of its STEs points at.
    pub(super) cd: u64,
    pub(super) streams: [u32; MAX_STREAMS],
    pub(super) count: usize,
}

impl Domain {
    pub(super) fn holds(&self, sid: u32) -> bool {
        self.streams[..self.count].contains(&sid)
    }

    pub(super) fn add(&mut self, sid: u32) -> bool {
        if self.holds(sid) {
            return true;
        }
        if self.count == MAX_STREAMS {
            return false;
        }
        self.streams[self.count] = sid;
        self.count += 1;
        true
    }

    pub(super) fn remove(&mut self, sid: u32) {
        if let Some(at) = self.streams[..self.count].iter().position(|&s| s == sid) {
            self.streams.copy_within(at + 1..self.count, at);
            self.count -= 1;
        }
    }
}

pub(super) fn slot(id: DomainId) -> Option<usize> {
    let raw = usize::from(id.as_u16());
    (1..=MAX_DOMAINS).contains(&raw).then(|| raw - 1)
}

pub(super) fn id(slot: usize) -> DomainId {
    DomainId::new(slot as u16 + 1)
}

/// The ASID a domain's TLB entries carry: its id, so two live domains never
/// share one and an invalidation by ASID reaches exactly one domain.
pub(super) fn asid(slot: usize) -> u16 {
    slot as u16 + 1
}
