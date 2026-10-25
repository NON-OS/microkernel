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

use super::node_ref::NodeRef;

/// A node and every ancestor up to the root, root first.
pub struct Chain<'a> {
    nodes: [NodeRef<'a>; MAX_DEPTH],
    len: usize,
}

impl<'a> Chain<'a> {
    pub fn as_slice(&self) -> &[NodeRef<'a>] {
        &self.nodes[..self.len]
    }
}

impl<'a> Fdt<'a> {
    pub fn chain(&self, node: &NodeRef) -> Result<Chain<'a>, FdtError> {
        let mut nodes = [NodeRef::EMPTY; MAX_DEPTH];
        for found in self.nodes() {
            let found = found?;
            // In document order the slot at a depth always holds the node
            // open there, so on reaching `node` the slots above it are its
            // ancestors.
            nodes[found.depth] = found;
            if found.same(node) {
                return Ok(Chain { nodes, len: found.depth + 1 });
            }
        }
        Err(FdtError::NoSuchNode)
    }
}
