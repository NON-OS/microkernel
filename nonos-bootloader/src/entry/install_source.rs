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

//! The two regions the installer writes to a disk, recorded for the kernel.

use alloc::vec::Vec;

use nonos_boot::handoff::types::{Module, MODULE_KIND_KERNEL_IMAGE, MODULE_KIND_LOADER_IMAGE};
use nonos_boot::loader::file::load_file_from_esp;
use uefi::prelude::*;

/*
 * The loader is recorded as the file the firmware read, not the image it
 * built from that file: what sits at LoadedImage's base is the PE after
 * section placement and relocation, a megabyte larger than BOOTX64.EFI and
 * not a bootable file. So the file is read again here, from the volume this
 * loader came from, into loader memory the kernel never reclaims. The
 * kernel image is the buffer that was read and verified above. A loader
 * whose file cannot be found records a zero region, and the installer then
 * says so instead of writing a disk with no bootloader on it.
 */
pub fn install_source(st: &SystemTable<Boot>, kernel_data: &[u8]) -> [Module; 2] {
    let loader = match load_file_from_esp(st, uefi::cstr16!("\\EFI\\BOOT\\BOOTX64.EFI")) {
        Ok(bytes) => region(Vec::leak(bytes), MODULE_KIND_LOADER_IMAGE),
        Err(_) => Module::default(),
    };
    [loader, region(kernel_data, MODULE_KIND_KERNEL_IMAGE)]
}

fn region(bytes: &[u8], kind: u32) -> Module {
    Module { base: bytes.as_ptr() as u64, size: bytes.len() as u64, kind, reserved: 0 }
}
