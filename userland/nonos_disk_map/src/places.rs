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

//! The fixed sectors, and what each one holds.

/// The unit of every LBA here. The kernel passes over a disk whose logical
/// blocks are larger (`src/hardware/block_device/fit.rs`).
pub const SECTOR_SIZE: usize = 512;

/// The package store's first sector: clear of the MBR and the primary GPT,
/// which end at sector 34.
pub const STORE_BASE_LBA: u64 = 256;

/// The sector past the store. The kernel serves store reads and writes only
/// below it, whatever the size of the disk.
pub const STORE_END_LBA: u64 = PLAN_LBA;

/// The disk plan: one plain sector naming the data volume and the files to
/// import, 32 MiB in.
pub const PLAN_LBA: u64 = 65_536;
pub const PLAN_MAGIC: [u8; 8] = *b"NONOSDP1";

/// The key header, the sector after the plan. A sector without its magic,
/// all zeros included, says a TPM keys the volume.
pub const KEY_LBA: u64 = PLAN_LBA + 1;
pub const KEY_MAGIC: [u8; 8] = *b"NONOSDK1";

/// Nothing the plan names may start below this sector, 64 MiB in.
pub const DATA_FLOOR: u64 = 131_072;

/// The data volume's header ring, its first sectors. The kernel formats a
/// volume only over a ring that is all zeros.
pub const HEADER_RING_SECTORS: u64 = 256;

/// The smallest volume the kernel's plan parser takes: the ring, a root and
/// one block more.
pub const MIN_VOLUME_SECTORS: u64 = HEADER_RING_SECTORS + 2;
