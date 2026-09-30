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

//! Where the kernel block layer looks for NONOS on a disk. The values are the
//! ones src/hardware/block_device/identify.rs checks: the package store header
//! capsule_vfs writes at LBA 256 (blk/store.rs STORE_BASE_LBA, magic in bytes
//! 0..8, blk/store_header.rs) and the disk plan blockfs_volume reads at
//! PLAN_LBA (src/fs/blockfs_volume/plan_types.rs, magic in bytes 0..8). No
//! crate the driver can depend on exports them, so they are restated here.

pub const STORE_LBA: u64 = 256;
pub const STORE_MAGIC: &[u8; 8] = b"NONOSTR1";
pub const PLAN_LBA: u64 = 65_536;
pub const PLAN_MAGIC: &[u8; 8] = b"NONOSDP1";

/// A disk of `capacity` sectors has sector `lba`. The kernel skips a structure
/// whose LBA is not below the capacity, and so does the probe.
pub const fn holds(capacity: u64, lba: u64) -> bool {
    lba < capacity
}

/// The sector starts with `magic`.
pub fn starts_with_magic(head: &[u8], magic: &[u8; 8]) -> bool {
    head.len() >= magic.len() && &head[..magic.len()] == magic
}
