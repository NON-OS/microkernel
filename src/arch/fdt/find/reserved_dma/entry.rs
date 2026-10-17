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

//! What is kept of one `iommu-addresses` entry. Free of the parser, so the
//! lookup proofs can mount it alone.

use crate::memory::iommu::reserved::ReservedRange;

/// Entries kept from the whole tree; more refuses the read.
pub const MAX_TREE_RANGES: usize = 16;
pub use crate::arch::fdt::find::iommus::MAX_STREAMS;

/// Whose range it is, as the remapping unit will see the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Owner {
    /// A PCI function the tree describes, by requester id.
    Pci { rid: u16 },
    /// A host bridge: every function on its buses shares the range.
    PciBridge { first_bus: u8, last_bus: u8 },
    /// A platform device, by its `iommus` stream ids.
    Streams { sids: [u32; MAX_STREAMS], count: u8 },
}

/// One entry: its owner, and its range or why it cannot be one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeRange {
    pub owner: Owner,
    pub range: ReservedRange,
    pub problem: Option<&'static [u8]>,
}

impl TreeRange {
    /// An unused slot.
    pub const EMPTY: Self = Self {
        owner: Owner::Streams { sids: [0; MAX_STREAMS], count: 0 },
        range: ReservedRange { iova: 0, phys: 0, len: 0 },
        problem: None,
    };
}

impl Owner {
    /// Whether a requester with `rid` (PCI) or stream `sid` is this owner.
    pub fn names_pci(&self, rid: u16) -> bool {
        match *self {
            Owner::Pci { rid: own } => own == rid,
            Owner::PciBridge { first_bus, last_bus } => {
                (first_bus..=last_bus).contains(&((rid >> 8) as u8))
            }
            Owner::Streams { .. } => false,
        }
    }

    pub fn names_stream(&self, sid: u32) -> bool {
        match *self {
            Owner::Streams { sids, count } => {
                sids[..usize::from(count).min(MAX_STREAMS)].contains(&sid)
            }
            _ => false,
        }
    }
}
