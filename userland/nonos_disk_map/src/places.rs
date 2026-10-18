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
//!
//! An LBA here counts 512-byte sectors on every disk, so a place is the
//! same byte offset whatever the disk's logical block. Every place starts
//! and ends on a block of `MAX_BLOCK_BYTES`, but for the key header, which
//! shares the plan's (`tests/four_k_places.rs`).

/// The unit of every LBA here, whatever the disk's logical block. A disk
/// of larger blocks is reached through its block layer's mapping: the
/// kernel's `src/hardware/nvme_capsule/client/lba_map.rs` `plan`, the
/// installer's `userland/nonos_blk_client/src/device/span.rs` `write`, the
/// USB driver's `userland/capsule_driver_usb_msc/src/span/mod.rs` `span`.
pub const SECTOR_SIZE: usize = 512;

/// The largest logical block a NONOS disk is laid out on. FAT32 takes no
/// larger sector for the ESP, and the loader reads the store only on a
/// block that divides its 4 KiB page
/// (`nonos-bootloader/src/loader/file/store_copy.rs` `copy_from`).
pub const MAX_BLOCK_BYTES: usize = 4096;

/// The package store's first sector, 128 KiB in: clear of the protective
/// MBR and the primary GPT on any block up to `MAX_BLOCK_BYTES`, which end
/// by sector 48 (at sector 34 on 512-byte blocks).
pub const STORE_BASE_LBA: u64 = 256;

/// The sector past the store, where a block starts. The kernel serves store
/// reads and writes only below it, whatever the size of the disk.
pub const STORE_END_LBA: u64 = PLAN_LBA;

/// The disk plan: one plain sector naming the data volume and the files to
/// import, 120 MiB in, the first sector of a block. It was 64 MiB in, just
/// past a full store of `MAX_TOTAL_BYTES`; the wallpapers now ride in the
/// store as entries vfs streams from the device (`STREAMED_MAX_BYTES`), and
/// they need the room.
pub const PLAN_LBA: u64 = 245_760;
pub const PLAN_MAGIC: [u8; 8] = *b"NONOSDP1";

/// The key header, the sector after the plan. A sector without its magic,
/// all zeros included, says a TPM keys the volume. On a disk of larger
/// blocks it lies in the plan's block, which holds nothing else: a write of
/// either one reads that block, patches its sector and writes it back.
pub const KEY_LBA: u64 = PLAN_LBA + 1;
pub const KEY_MAGIC: [u8; 8] = *b"NONOSDK1";

/// Nothing the plan names may start below this sector, 128 MiB in.
pub const DATA_FLOOR: u64 = 262_144;

/// The data volume's header ring, its first sectors, whole blocks of
/// `MAX_BLOCK_BYTES`. The kernel formats a volume only over a ring that is
/// all zeros.
pub const HEADER_RING_SECTORS: u64 = 256;

/// The smallest volume the kernel's plan parser takes: the ring, a root and
/// one block more. A length the parser checks, not a place, so it need not
/// be whole blocks of `MAX_BLOCK_BYTES`.
pub const MIN_VOLUME_SECTORS: u64 = HEADER_RING_SECTORS + 2;
