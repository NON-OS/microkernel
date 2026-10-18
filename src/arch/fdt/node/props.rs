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
use crate::arch::fdt::property::Property;
use crate::arch::fdt::walker::Event;

use super::cells::Cells;
use super::node_ref::NodeRef;

impl<'a> Fdt<'a> {
    /// The node's own properties, which all come before its first subnode.
    pub fn properties(
        &self,
        node: &NodeRef,
    ) -> impl Iterator<Item = Result<Property<'a>, FdtError>> {
        let mut walker = self.walker_at(node.body, 1);
        let mut done = false;
        core::iter::from_fn(move || {
            if done {
                return None;
            }
            match walker.next() {
                Ok(Some(Event::Property(prop))) => Some(Ok(prop)),
                Ok(_) => {
                    done = true;
                    None
                }
                Err(e) => {
                    done = true;
                    Some(Err(e))
                }
            }
        })
    }

    pub fn property(&self, node: &NodeRef, name: &[u8]) -> Result<Option<Property<'a>>, FdtError> {
        for prop in self.properties(node) {
            let prop = prop?;
            if prop.name == name {
                return Ok(Some(prop));
            }
        }
        Ok(None)
    }

    /// The counts the node sets for its own children.
    pub fn own_cells(&self, node: &NodeRef) -> Result<Cells, FdtError> {
        let mut cells = Cells::DEFAULT;
        for prop in self.properties(node) {
            cells.take(&prop?);
        }
        Ok(cells)
    }
}
