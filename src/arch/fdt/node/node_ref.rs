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

use super::cells::Cells;

/// A node found by a walk: enough to come back to its properties and to read
/// its `reg` without walking from the root again.
#[derive(Clone, Copy, Debug)]
pub struct NodeRef<'a> {
    pub name: &'a [u8],
    /// Structure offset of the node's first token after its name.
    pub body: usize,
    /// The root is 0.
    pub depth: usize,
    /// The parent's cell counts, which are how this node's `reg` reads.
    pub cells: Cells,
}

impl<'a> NodeRef<'a> {
    pub const EMPTY: NodeRef<'static> =
        NodeRef { name: &[], body: 0, depth: 0, cells: Cells::DEFAULT };

    /// The name without its unit address: `serial` for `serial@7e201000`.
    pub fn base_name(&self) -> &'a [u8] {
        match self.name.iter().position(|&b| b == b'@') {
            Some(at) => &self.name[..at],
            None => self.name,
        }
    }

    /// Whether a path component names this node. A component without a unit
    /// address matches on the base name, as the spec allows.
    pub fn named(&self, component: &[u8]) -> bool {
        self.name == component || (!component.contains(&b'@') && self.base_name() == component)
    }

    pub fn same(&self, other: &NodeRef) -> bool {
        self.body == other.body
    }
}
