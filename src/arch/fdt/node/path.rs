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

impl<'a> Fdt<'a> {
    /// The node at an absolute path such as `/soc/serial@7e201000`.
    pub fn find_path(&self, path: &[u8]) -> Result<Option<NodeRef<'a>>, FdtError> {
        if path.first() != Some(&b'/') {
            return Ok(None);
        }
        self.find_joined(path, &[])
    }

    /// The node at `head` followed by the relative `tail`, which is how an
    /// alias with a path after it resolves without a buffer to join them in.
    pub(super) fn find_joined(
        &self,
        head: &[u8],
        tail: &[u8],
    ) -> Result<Option<NodeRef<'a>>, FdtError> {
        let mut parts: [&[u8]; MAX_DEPTH] = [&[]; MAX_DEPTH];
        let mut count = 0usize;
        for part in head.split(|&b| b == b'/').chain(tail.split(|&b| b == b'/')) {
            if part.is_empty() {
                continue;
            }
            // Deeper than any tree we accept, so it names nothing.
            if count == MAX_DEPTH {
                return Ok(None);
            }
            parts[count] = part;
            count += 1;
        }
        // Nodes come in document order, so a node at depth d ends every
        // branch deeper than it: whatever matched below d - 1 is gone.
        let mut matched = 0usize;
        for node in self.nodes() {
            let node = node?;
            if node.depth == 0 {
                if count == 0 {
                    return Ok(Some(node));
                }
                continue;
            }
            matched = matched.min(node.depth - 1);
            if matched == node.depth - 1 && matched < count && node.named(parts[matched]) {
                matched += 1;
                if matched == count {
                    return Ok(Some(node));
                }
            }
        }
        Ok(None)
    }
}
