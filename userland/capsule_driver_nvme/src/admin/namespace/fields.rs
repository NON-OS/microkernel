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

//! The namespace fields the driver keeps, and the absent namespace.

#[derive(Clone, Copy)]
pub struct NamespaceIdentity {
    pub nsid: u32,
    pub size_lba: u64,
    pub capacity_lba: u64,
    pub used_lba: u64,
    pub lba_size: u32,
    pub metadata_size: u16,
    pub format_index: u8,
    /// FLBAS bits 6:5. NVMe 2.0 uses them as the upper bits of the format
    /// index when a namespace has more than 16 formats; the parser reads only
    /// the 16 slots the low four bits reach.
    pub format_index_upper: u8,
    pub formatted_lba_count: u8,
}

impl NamespaceIdentity {
    pub const fn absent() -> Self {
        Self {
            nsid: 0,
            size_lba: 0,
            capacity_lba: 0,
            used_lba: 0,
            lba_size: 0,
            metadata_size: 0,
            format_index: 0,
            format_index_upper: 0,
            formatted_lba_count: 0,
        }
    }
}
