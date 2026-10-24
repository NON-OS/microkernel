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

//! A node's ID mappings, each read and checked whole.

use super::error::IortError;
use super::layout::{
    MAP_COUNT, MAP_FLAGS, MAP_INPUT, MAP_OUTPUT, MAP_REFERENCE, MAP_SINGLE, MAP_SIZE,
    NODE_MAPPINGS, NODE_MAPPING_ARRAY,
};
use super::node::Node;
use super::read::{index, u32_at};

/// IDs `[input, input + length)` leave as `[output, output + length)` at
/// the node `target` (an offset from the table's start).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Mapping {
    pub input: u32,
    pub length: u32,
    pub output: u32,
    pub target: usize,
    pub single: bool,
}

/// Mapping `n` of `node`. The table stores the count minus one; a range
/// that would wrap past 2^32 on either side is refused, not clipped.
pub(super) fn mapping(node: &Node, n: usize) -> Result<Mapping, IortError> {
    let array = index(node.bytes, NODE_MAPPING_ARRAY)?;
    let at = n.checked_mul(MAP_SIZE).and_then(|o| o.checked_add(array));
    let at = at.ok_or(IortError::Short)?;
    let field = |offset| u32_at(node.bytes, at + offset);
    let (input, output) = (field(MAP_INPUT)?, field(MAP_OUTPUT)?);
    let length = field(MAP_COUNT)?.checked_add(1).ok_or(IortError::BadMapping)?;
    if input.checked_add(length - 1).is_none() || output.checked_add(length - 1).is_none() {
        return Err(IortError::BadMapping);
    }
    let target = usize::try_from(field(MAP_REFERENCE)?).map_err(|_| IortError::Short)?;
    let single = field(MAP_FLAGS)? & MAP_SINGLE != 0;
    Ok(Mapping { input, length, output, target, single })
}

/// How many mappings `node` declares; each is read by `mapping`, which
/// fails on the first that lies past the node's end.
pub(super) fn count(node: &Node) -> Result<usize, IortError> {
    index(node.bytes, NODE_MAPPINGS)
}
