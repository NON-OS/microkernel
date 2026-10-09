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
//!   LBA 0 .. 34          protective MBR, primary GPT header and entries
//!   LBA 256 .. 245760    the package store: vfs loads it and appends to it
//!   LBA 245760           the disk plan: where the data volume lies
//!   LBA 245761           the key header: how the volume key is reached
//!   LBA 262144 ..        the data volume, and anything the plan imports
//! ```
//!
//! The kernel settles on the disk that carries the store's magic at 256 or
//! the plan's at 245760 (`src/hardware/block_device/identify.rs`), serves
//! store reads and writes only inside 256..245760
//! (`src/syscall/microkernel/store_read.rs`, `store_write.rs`), and reads the
//! plan and the key header itself (`src/fs/blockfs_volume/`). It keeps its
//! own copies of these numbers, and so do the host tools that pack a store
//! and write a plan; `tests/` reads those sources and fails when one
//! differs from what is here.

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
    DATA_FLOOR, HEADER_RING_SECTORS, KEY_LBA, KEY_MAGIC, MIN_VOLUME_SECTORS, PLAN_LBA, PLAN_MAGIC,
    SECTOR_SIZE, STORE_BASE_LBA, STORE_END_LBA,
};
