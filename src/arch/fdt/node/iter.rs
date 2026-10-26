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
use crate::arch::fdt::limits::MAX_DEPTH;
use crate::arch::fdt::parser::Fdt;
use crate::arch::fdt::walker::{Event, Walker};

use super::cells::Cells;
use super::node_ref::NodeRef;

/// Every node in document order, each carrying its parent's cell counts. The
/// counts are known by the time a child appears because validation refused
/// any property that follows a subnode.
pub struct NodeIter<'a> {
    walker: Walker<'a>,
    /// Cell counts of the node open at each depth, by nodes-open count.
    stack: [Cells; MAX_DEPTH + 1],
    failed: bool,
}

impl<'a> Fdt<'a> {
    pub fn nodes(&self) -> NodeIter<'a> {
        NodeIter { walker: self.walker(), stack: [Cells::DEFAULT; MAX_DEPTH + 1], failed: false }
    }
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = Result<NodeRef<'a>, FdtError>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.failed {
            let event = match self.walker.next_event() {
                Ok(Some(event)) => event,
                Ok(None) => return None,
                Err(e) => {
                    self.failed = true;
                    return Some(Err(e));
                }
            };
            let open = self.walker.depth();
            match event {
                Event::BeginNode { name, body } => {
                    let cells = self.stack[open - 1];
                    self.stack[open] = Cells::DEFAULT;
                    return Some(Ok(NodeRef { name, body, depth: open - 1, cells }));
                }
                Event::Property(prop) => self.stack[open].take(&prop),
                Event::EndNode => {}
            }
        }
        None
    }
}
