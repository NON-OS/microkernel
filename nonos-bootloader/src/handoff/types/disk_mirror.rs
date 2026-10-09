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

/*
 * Ranges of the boot disk the loader copied into memory for the kernel: the
 * live disk plan's sector and every file that plan names for import (the
 * release stick's Qwen tier). The kernel answers reads of those sectors from
 * the copy when none of its own drivers drives the disk, so a live session
 * imports its model whatever USB controller or stick the machine has. The
 * record reaches the kernel as a module of kind `MODULE_KIND_DISK_MIRROR`;
 * its layout matches the kernel's `DiskMirror` byte for byte.
 */

/* The record's first eight bytes. */
pub const DISK_MIRROR_MAGIC: [u8; 8] = *b"NONOSDM1";
/* The plan's sector and the 30 imports a plan may name. */
pub const MIRROR_EXTENTS: usize = 31;
pub const MODULE_KIND_DISK_MIRROR: u32 = 8;

/* Sectors `lba..lba + sectors` (512 bytes each) of the disk, at `phys`. */
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct MirrorExtent {
    pub lba: u64,
    pub sectors: u64,
    pub phys: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DiskMirror {
    pub magic: [u8; 8],
    /* The whole disk's size in 512-byte sectors, as its plan is checked against. */
    pub capacity: u64,
    pub count: u64,
    pub extents: [MirrorExtent; MIRROR_EXTENTS],
}

pub const DISK_MIRROR_LEN: usize = 24 + 24 * MIRROR_EXTENTS;

const _: () = assert!(core::mem::size_of::<DiskMirror>() == DISK_MIRROR_LEN);
