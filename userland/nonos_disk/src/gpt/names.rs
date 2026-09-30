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

//! What the table says about each partition: its name, the type that tells
//! firmware and partition tools what it is, and its attribute bits.

use crate::guid::Guid;
use crate::layout::Region;

/// Attribute bit 1, "No Block IO Protocol" (UEFI 2.10, section 5.3.3, the
/// partition entry attributes): firmware must not produce a Block IO device
/// for the partition, so it does not probe the store, the plan or the
/// sealed volume for a filesystem.
const NO_BLOCK_IO: u64 = 1 << 1;

impl Region {
    /// The name partition tools and firmware menus show. Fixed, so a disk
    /// NONOS wrote can be told from one something else wrote by its entries.
    pub fn name(self) -> &'static str {
        match self {
            Region::Store => "NONOS-STORE",
            Region::Plan => "NONOS-PLAN",
            Region::Data => "NONOS-DATA",
            Region::Esp => "NONOS-ESP",
        }
    }

    pub fn type_guid(self) -> Guid {
        match self {
            Region::Store => Guid::NONOS_STORE,
            Region::Plan => Guid::NONOS_PLAN,
            Region::Data => Guid::NONOS_DATA,
            Region::Esp => Guid::ESP,
        }
    }

    /// The ESP carries none: not required, not legacy bootable, and seen by
    /// firmware, which is the point of it.
    pub fn attributes(self) -> u64 {
        match self {
            Region::Esp => 0,
            _ => NO_BLOCK_IO,
        }
    }
}
