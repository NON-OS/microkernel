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

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

/// One interrupt: the controller it belongs to and that controller's cells,
/// still big-endian, for the controller's own binding to decode.
#[derive(Clone, Copy, Debug)]
pub struct Specifier<'a> {
    pub controller: NodeRef<'a>,
    pub cells: &'a [u8],
}

impl<'a> Fdt<'a> {
    /// The node's `index`th interrupt, from `interrupts-extended` when it has
    /// one (each entry names its own controller) and from `interrupts` and
    /// the interrupt parent otherwise.
    pub fn interrupt(
        &self,
        node: &NodeRef,
        index: usize,
    ) -> Result<Option<Specifier<'a>>, FdtError> {
        if let Some(ext) = self.property(node, b"interrupts-extended")? {
            let mut at = 0usize;
            let mut seen = 0usize;
            while at < ext.data.len() {
                let controller = self.by_phandle(be_u32(ext.data, at)?)?;
                let len = self.interrupt_cells(&controller)? as usize * 4;
                let cells = ext.data.get(at + 4..at + 4 + len).ok_or(FdtError::BadCells)?;
                if seen == index {
                    return Ok(Some(Specifier { controller, cells }));
                }
                seen += 1;
                at += 4 + len;
            }
            return Ok(None);
        }
        let Some(prop) = self.property(node, b"interrupts")? else {
            return Ok(None);
        };
        let controller = self.interrupt_parent(node)?.ok_or(FdtError::NoSuchNode)?;
        let len = self.interrupt_cells(&controller)? as usize * 4;
        if prop.data.len() % len != 0 {
            return Err(FdtError::BadCells);
        }
        Ok(prop.data.chunks_exact(len).nth(index).map(|cells| Specifier { controller, cells }))
    }
}
