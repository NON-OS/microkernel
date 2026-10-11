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

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::parser::Fdt;
use crate::arch::fdt::property::RegIter;

use super::node_ref::NodeRef;

impl<'a> Fdt<'a> {
    /// The node's `reg` entries as its parent bus numbers them.
    pub fn reg(&self, node: &NodeRef) -> Result<Option<RegIter<'a>>, FdtError> {
        Ok(self.property(node, b"reg")?.map(|p| p.reg_iter(node.cells.address, node.cells.size)))
    }

    /// The `index`th `reg` entry as a CPU physical `(base, size)`. `None` when
    /// the node has fewer entries; an entry that cannot be read or mapped is
    /// an error, because a device placed at a guess is worse than none.
    pub fn mmio(&self, node: &NodeRef, index: usize) -> Result<Option<(u64, u64)>, FdtError> {
        let Some(mut reg) = self.reg(node)? else {
            return Ok(None);
        };
        if !reg.readable() {
            return Err(FdtError::BadCells);
        }
        let Some((addr, size)) = reg.nth(index) else {
            return Ok(None);
        };
        let base = self.translate(node, addr, size)?;
        base.checked_add(size).ok_or(FdtError::OutOfBounds)?;
        Ok(Some((base, size)))
    }
}
