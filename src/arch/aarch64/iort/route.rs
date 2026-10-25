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

//! One map entry added, and one unit chosen, per root complex.

use super::error::IortError;
use super::node::Node;
use crate::arch::fdt::find::smmu::{IommuMap, MapEntry, MAX_MAP_ENTRIES};

pub(super) const EMPTY: IommuMap = IommuMap {
    entries: [MapEntry { rid_base: 0, phandle: 0, sid_base: 0, length: 0 }; MAX_MAP_ENTRIES],
    count: 0,
    mask: u32::MAX,
};

/// IORT has no mask: every requester id is looked up as it is.
pub(super) fn push(map: &mut IommuMap, entry: MapEntry) -> Result<(), IortError> {
    let slot = map.entries.get_mut(map.count).ok_or(IortError::TooMany)?;
    *slot = entry;
    map.count += 1;
    Ok(())
}

/// The unit a root complex sends to, which must be the same for all of its
/// mappings of that kind.
pub(super) fn one(slot: &mut Option<u32>, handle: u32) -> Result<(), IortError> {
    match *slot {
        Some(held) if held != handle => Err(IortError::Ambiguous),
        _ => {
            *slot = Some(handle);
            Ok(())
        }
    }
}

/// What the records name a node by, where the tree uses a phandle. A node
/// lies past the header, so its offset is never zero, the tree's "none".
pub(super) fn handle(node: &Node) -> Result<u32, IortError> {
    u32::try_from(node.offset).map_err(|_| IortError::Short)
}
