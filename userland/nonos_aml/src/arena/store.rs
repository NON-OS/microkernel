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

//! The arena: nodes, and one shared list of their operands.

use super::{Limits, Node};
use alloc::vec::Vec;

/// Nodes refer to each other by index. Operands are gathered on `stack` while
/// their object is read, then moved to `kids` in one run when it is finished,
/// so every node's operands are contiguous without a list per node.
#[derive(Debug, Default)]
pub struct Arena {
    pub(super) nodes: Vec<Node>,
    pub(super) kids: Vec<u32>,
    pub(super) stack: Vec<u32>,
    pub limits: Limits,
}

impl Arena {
    pub fn new(limits: Limits) -> Self {
        Self { nodes: Vec::new(), kids: Vec::new(), stack: Vec::new(), limits }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn node(&self, i: u32) -> Option<&Node> {
        self.nodes.get(i as usize)
    }

    /// The operands of node `i`, in grammar order.
    pub fn kids(&self, i: u32) -> &[u32] {
        let span = self.node(i).map(|n| n.kids).unwrap_or_default();
        &self.kids[span.at as usize..span.end() as usize]
    }
}
