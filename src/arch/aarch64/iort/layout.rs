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

//! Where IORT keeps what this reader takes, from Arm's IO Remapping Table
//! specification (DEN0049): offsets from the table's start, or from a
//! node's. Recalled, not checked against a copy in this tree; the letter
//! lists every value for the owner.

/// After the 36-byte ACPI header: the node count, then the node array's
/// offset.
pub(super) const NODE_COUNT: usize = 36;
pub(super) const NODE_ARRAY: usize = 40;

/// Every node: type (u8), length (u16), revision (u8), identifier (u32),
/// ID mapping count (u32), ID array offset from the node (u32).
pub(super) const NODE_TYPE: usize = 0;
pub(super) const NODE_LENGTH: usize = 1;
pub(super) const NODE_MAPPINGS: usize = 8;
pub(super) const NODE_MAPPING_ARRAY: usize = 12;
pub(super) const NODE_HEADER: usize = 16;

pub(super) const ITS_GROUP: u8 = 0;
pub(super) const ROOT_COMPLEX: u8 = 2;
pub(super) const SMMU_V3: u8 = 4;

/// An ITS group: the ITS count, then that many ITS identifiers (u32).
pub(super) const ITS_COUNT: usize = 16;
pub(super) const ITS_IDS: usize = 20;

/// A root complex: its PCI segment, after the memory access properties (8)
/// and the ATS attribute (4).
pub(super) const RC_SEGMENT: usize = 28;
pub(super) const RC_MIN: usize = 32;

/// An SMMUv3: base (u64), flags (u32), then after VATOS and model, the
/// event and global-error GSIVs; the DeviceID mapping index from revision 1.
pub(super) const SMMU_BASE: usize = 16;
pub(super) const SMMU_FLAGS: usize = 24;
pub(super) const SMMU_EVENT_GSIV: usize = 44;
pub(super) const SMMU_GERR_GSIV: usize = 52;
pub(super) const SMMU_MIN: usize = 60;
pub(super) const SMMU_DEVICE_ID_INDEX: usize = 64;
/// Flags: COHACC override (table walks coherent), DeviceID index valid.
pub(super) const SMMU_COHERENT: u32 = 1 << 0;
pub(super) const SMMU_INDEX_VALID: u32 = 1 << 4;
/// Pages 0 and 1, each 64 KiB: what the SMMUv3 driver maps.
pub(super) const SMMU_WINDOW: u64 = 0x2_0000;

/// One ID mapping, 20 bytes: input base, number of IDs minus one, output
/// base, output node's offset from the table, flags (bit 0 single mapping).
pub(super) const MAP_INPUT: usize = 0;
pub(super) const MAP_COUNT: usize = 4;
pub(super) const MAP_OUTPUT: usize = 8;
pub(super) const MAP_REFERENCE: usize = 12;
pub(super) const MAP_FLAGS: usize = 16;
pub(super) const MAP_SIZE: usize = 20;
pub(super) const MAP_SINGLE: u32 = 1 << 0;
