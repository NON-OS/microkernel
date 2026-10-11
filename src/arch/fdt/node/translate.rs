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

//! Bus addresses to CPU physical addresses, through each parent's `ranges`.
//! The Pi quotes everything under `/soc` at its VideoCore bus address
//! (0x7e20_1000); only `ranges` says the CPU sees it at 0xfe20_1000.

use crate::arch::fdt::endian::be_cells;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;

use super::cells::Cells;
use super::node_ref::NodeRef;

impl<'a> Fdt<'a> {
    /// Where the CPU sees `size` bytes at `addr` in the node's `reg` space.
    /// The span has to fit one window at every level; a bus without `ranges`
    /// is not memory mapped.
    pub fn translate(&self, node: &NodeRef, addr: u64, size: u64) -> Result<u64, FdtError> {
        let chain = self.chain(node)?;
        let chain = chain.as_slice();
        let mut addr = addr;
        // chain[i] is a bus; chain[i + 1].cells are its own counts and
        // chain[i].cells its parent's. The root is not a bus to cross.
        for i in (1..node.depth).rev() {
            let Some(ranges) = self.property(&chain[i], b"ranges")? else {
                return Err(FdtError::NotTranslatable);
            };
            addr = window(ranges.data, chain[i + 1].cells, chain[i].cells, addr, size)?;
        }
        Ok(addr)
    }
}

/// One level: `ranges` entries are (child address, parent address, size). An
/// empty property maps the bus one to one.
fn window(data: &[u8], own: Cells, parent: Cells, addr: u64, size: u64) -> Result<u64, FdtError> {
    if data.is_empty() {
        return Ok(addr);
    }
    own.fit_u64()?;
    parent.fit_u64()?;
    let parent_at = own.address as usize * 4;
    let size_at = parent_at + parent.address as usize * 4;
    let stride = size_at + own.size as usize * 4;
    for entry in data.chunks_exact(stride) {
        let child = be_cells(entry, 0, own.address)?;
        let base = be_cells(entry, parent_at, parent.address)?;
        let span = be_cells(entry, size_at, own.size)?;
        // A window running off the top of either space is broken, not a map.
        let last = span.saturating_sub(1);
        if child.checked_add(last).is_none() || base.checked_add(last).is_none() {
            continue;
        }
        let Some(offset) = addr.checked_sub(child) else { continue };
        // The span has to end inside the window, not merely start there.
        if offset < span && size <= span - offset {
            return base.checked_add(offset).ok_or(FdtError::NotTranslatable);
        }
    }
    Err(FdtError::NotTranslatable)
}
