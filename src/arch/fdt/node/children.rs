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
use crate::arch::fdt::walker::Event;

use super::node_ref::NodeRef;

impl<'a> Fdt<'a> {
    /// The node's direct subnodes, in order.
    pub fn children(
        &self,
        node: &NodeRef,
    ) -> Result<impl Iterator<Item = Result<NodeRef<'a>, FdtError>>, FdtError> {
        let cells = self.own_cells(node)?;
        let depth = node.depth + 1;
        let mut walker = self.walker_at(node.body, 1);
        let mut done = false;
        Ok(core::iter::from_fn(move || {
            while !done {
                match walker.next_event() {
                    Ok(Some(Event::BeginNode { name, body })) if walker.depth() == 2 => {
                        return Some(Ok(NodeRef { name, body, depth, cells }));
                    }
                    Ok(Some(_)) if walker.depth() == 0 => done = true,
                    Ok(Some(_)) => {}
                    Ok(None) => done = true,
                    Err(e) => {
                        done = true;
                        return Some(Err(e));
                    }
                }
            }
            None
        }))
    }
}
