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
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

impl<'a> Fdt<'a> {
    /// The controller the node's `interrupts` are numbered for: the nearest
    /// `interrupt-parent`, on the node or else on an ancestor. `None` when no
    /// node on the way up names one.
    pub fn interrupt_parent(&self, node: &NodeRef) -> Result<Option<NodeRef<'a>>, FdtError> {
        let chain = self.chain(node)?;
        for hop in chain.as_slice().iter().rev() {
            if let Some(parent) = self.property(hop, b"interrupt-parent")? {
                if parent.data.len() != 4 {
                    return Err(FdtError::BadCells);
                }
                return self.by_phandle(parent.u32()?).map(Some);
            }
        }
        Ok(None)
    }

    /// The controller's `#interrupt-cells`, which has to be at least one.
    pub fn interrupt_cells(&self, controller: &NodeRef) -> Result<u32, FdtError> {
        match self.property(controller, b"#interrupt-cells")? {
            Some(p) => match p.cell_count()? {
                0 => Err(FdtError::BadCells),
                n => Ok(n),
            },
            None => Err(FdtError::BadCells),
        }
    }
}
