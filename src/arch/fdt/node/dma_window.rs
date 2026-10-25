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

//! Where a device's bus masters land in CPU physical memory, through each
//! parent bus's `dma-ranges` (Devicetree Specification v0.4, 2.3.9).
//!
//! The Pi 4's eMMC2 sees RAM's first gigabyte at bus address 0xc000_0000,
//! so a descriptor holding a physical address sends its transfer elsewhere.
//! An empty `dma-ranges` is one to one; the specification gives no meaning
//! to a bus without one, and boards' trees leave it off where it is one to
//! one, so that is read as one to one too.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;

use super::dma_narrow::narrow;
use super::node_ref::NodeRef;

/// Bus addresses `[bus_base, bus_base + size)` reach CPU physical memory at
/// `[cpu_base, cpu_base + size)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DmaWindow {
    pub bus_base: u64,
    pub cpu_base: u64,
    pub size: u64,
}

impl DmaWindow {
    pub const IDENTITY: Self = Self { bus_base: 0, cpu_base: 0, size: u64::MAX };

    /// The bus address for `[phys, phys + len)`, when all of it is inside.
    pub fn bus_address(&self, phys: u64, len: u64) -> Option<u64> {
        let offset = phys.checked_sub(self.cpu_base)?;
        if offset > self.size || len > self.size - offset {
            return None;
        }
        self.bus_base.checked_add(offset)
    }
}

impl<'a> Fdt<'a> {
    /// The window the node's DMA reaches memory through, narrowed at every
    /// bus on the way to the root. `None` when some bus maps none of it.
    pub fn dma_window(&self, node: &NodeRef) -> Result<Option<DmaWindow>, FdtError> {
        let chain = self.chain(node)?;
        let chain = chain.as_slice();
        let mut window = DmaWindow::IDENTITY;
        // As in `translate`: chain[i] is a bus, chain[i + 1].cells its own
        // counts and chain[i].cells its parent's.
        for i in (1..node.depth).rev() {
            let Some(ranges) = self.property(&chain[i], b"dma-ranges")? else {
                continue;
            };
            match narrow(ranges.data, chain[i + 1].cells, chain[i].cells, window)? {
                Some(next) => window = next,
                None => return Ok(None),
            }
        }
        Ok(Some(window))
    }
}
