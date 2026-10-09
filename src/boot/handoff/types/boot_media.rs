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
 * The partition the loader was read from, as the firmware's device path
 * names it: the loader records it as a module of kind
 * `MODULE_KIND_BOOT_MEDIA`, and `MkInstallSource` hands the record to the
 * installer, which leaves out the disk whose table it describes (the stick
 * the machine booted from). Layout matches the loader's `BootMedia`.
 */
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BootMedia {
    /* Index of the partition in its table, from 1. */
    pub partition_number: u32,
    /* 1 for an MBR, 2 for a GPT, 0 when the firmware named no partition. */
    pub table: u8,
    /* 1: the MBR disk signature, 2: the GPT partition GUID, 0: none. */
    pub signature_type: u8,
    pub reserved: u16,
    pub start_lba: u64,
    pub size_lba: u64,
    /* The GUID as stored on disk, or the MBR signature in the first four. */
    pub signature: [u8; 16],
}

pub const MODULE_KIND_BOOT_MEDIA: u32 = 3;
pub const BOOT_MEDIA_LEN: usize = 40;

const _: () = assert!(core::mem::size_of::<BootMedia>() == BOOT_MEDIA_LEN);
