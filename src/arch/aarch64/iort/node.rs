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

//! The node array, walked with every node's length checked against the
//! table, so nothing after reads outside the node it was handed.

use super::error::IortError;
use super::layout::{NODE_ARRAY, NODE_COUNT, NODE_HEADER, NODE_LENGTH, NODE_TYPE};
use super::read::{index, u16_at, u8_at};

/// One node: its offset from the table's start, which is how ID mappings
/// name it, its type, and its bytes.
#[derive(Debug, Clone, Copy)]
pub(super) struct Node<'t> {
    pub offset: usize,
    pub kind: u8,
    pub bytes: &'t [u8],
}

fn node_at(table: &[u8], offset: usize) -> Result<Node<'_>, IortError> {
    let kind = u8_at(table, offset + NODE_TYPE)?;
    let length = usize::from(u16_at(table, offset + NODE_LENGTH)?);
    if length < NODE_HEADER {
        return Err(IortError::BadNode);
    }
    let end = offset.checked_add(length).ok_or(IortError::Short)?;
    let bytes = table.get(offset..end).ok_or(IortError::Short)?;
    Ok(Node { offset, kind, bytes })
}

/// Every node, in table order. The array must start after the header's
/// own fields, and each node follows the last: the count is bounded by the
/// table's length because every node is at least a header long.
pub(super) fn each<'t>(
    table: &'t [u8],
    mut visit: impl FnMut(Node<'t>) -> Result<(), IortError>,
) -> Result<(), IortError> {
    let count = index(table, NODE_COUNT)?;
    let mut offset = index(table, NODE_ARRAY)?;
    if offset < NODE_ARRAY + 8 {
        return Err(IortError::BadNode);
    }
    for _ in 0..count {
        let node = node_at(table, offset)?;
        offset += node.bytes.len();
        visit(node)?;
    }
    Ok(())
}

/// The node at `offset`, which must be one the array walk reaches: an ID
/// mapping that points between nodes names nothing.
pub(super) fn find(table: &[u8], offset: usize) -> Result<Node<'_>, IortError> {
    let mut found = None;
    each(table, |node| {
        if node.offset == offset {
            found = Some(node);
        }
        Ok(())
    })?;
    found.ok_or(IortError::BadMapping)
}
