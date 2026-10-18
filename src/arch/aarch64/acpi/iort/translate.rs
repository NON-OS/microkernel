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

//! A device's ID followed through the mappings, node to node, until it
//! reaches an ITS group: the group and the ID the ITS sees. A range that
//! holds the ID is taken first; failing one, a single mapping gives the
//! node's own output ID, whatever came in. At most as many
//! steps as there are nodes, so a table whose mappings loop answers None
//! rather than spinning.

use super::node::{Kind, Node};

/// From the node at `from` with input ID `id`: the ITS group's offset and
/// the device ID the ITS is given, or None when no mapping holds the ID,
/// a mapping names no node, or the chain loops.
pub fn translate(nodes: &[Node], from: u32, id: u32) -> Option<(u32, u32)> {
    let find = |at: u32| nodes.binary_search_by_key(&at, |n| n.offset).ok().map(|i| &nodes[i]);
    let (mut node, mut id) = (find(from)?, id);
    for _ in 0..=nodes.len() {
        if matches!(node.kind, Kind::ItsGroup { .. }) {
            return Some((node.offset, id));
        }
        let range = node.mappings.iter().find_map(|m| m.map(id).map(|o| (m.output_ref, o)));
        let single =
            || node.mappings.iter().find(|m| m.single).map(|m| (m.output_ref, m.output_base));
        let (to, out) = range.or_else(single)?;
        (node, id) = (find(to)?, out);
    }
    None
}
