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

//! Where a NONOS disk keeps what, in 512-byte sectors.
//!
//! ```text
//!   LBA 0 .. 34          protective MBR, primary GPT (0 .. 48 on 4 KiB blocks)
//!   LBA 256 .. 245760    the package store: vfs loads it and appends to it
//!   LBA 245760           the disk plan: where the data volume lies
//!   LBA 245761           the key header: how the volume key is reached
//!   LBA 262144 ..        the data volume, and anything the plan imports
//! ```
//!
//! An LBA here counts 512-byte sectors on every disk, so each place is the
//! same byte offset whatever the disk's logical block, and a disk of larger
//! blocks is reached through its block layer's mapping. The protective MBR
//! and the GPT are the firmware's, which reads them in the disk's own
//! blocks. Every place here starts and ends on a block of
//! [`MAX_BLOCK_BYTES`] but the key header, which shares the plan's.
//!
//! The kernel settles on the disk that carries the store's magic or the
//! plan's where its GPT's NONOS partitions start, or at 256 and 245760 on a
//! disk whose table names none (`src/hardware/block_device/identify.rs`,
//! `gpt/fixed.rs`), preferring the disk the loader came from. It serves
//! store reads and writes only inside 256..245760
//! (`src/syscall/microkernel/store_read.rs`, `store_write.rs`), and reads the
//! plan and the key header itself (`src/fs/blockfs_volume/`). It keeps its
//! own copies of these numbers. So does the bootloader, which copies the
//! store and reads a live plan before the kernel runs
//! (`nonos-bootloader/src/loader/file/store_copy.rs`, `disk_mirror/plan.rs`),
//! the AHCI driver, which picks its disk by the same two magics, fs_proofs'
//! stand-in for the kernel's store calls, and the host tools that pack a
//! store and write a plan. `tests/` reads each of those sources and fails
//! when a number there differs from what is here.

#![no_std]

mod container;
mod digest;
mod places;

pub use container::{
    streamed, valid_name, ENTRY_LEN, HEADER_LEN, MAX_ENTRIES, MAX_TOTAL_BYTES, NAME_LEN,
    STORE_MAGIC, STORE_VERSION, STREAMED_MAX_BYTES, STREAMED_PREFIX, TOC_SPAN,
};
pub use digest::digest16;
pub use places::{
    DATA_FLOOR, HEADER_RING_SECTORS, KEY_LBA, KEY_MAGIC, MAX_BLOCK_BYTES, MIN_VOLUME_SECTORS,
    PLAN_LBA, PLAN_MAGIC, SECTOR_SIZE, STORE_BASE_LBA, STORE_END_LBA,
};
