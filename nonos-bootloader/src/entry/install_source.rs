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

//! The two regions the installer writes to a disk, and the partition the
//! loader came from, recorded for the kernel.

use alloc::boxed::Box;
use alloc::vec::Vec;

use nonos_boot::handoff::types::{
    InstallHandoff, Module, BOOT_MEDIA_LEN, MODULE_KIND_BOOT_MEDIA, MODULE_KIND_DISK_MIRROR,
    MODULE_KIND_KERNEL_IMAGE, MODULE_KIND_LOADER_IMAGE, MODULE_KIND_STORE,
};
use nonos_boot::loader::file::{boot_partition, disk_mirror, load_file_from_esp, store_copy};
use nonos_boot::menu::BootIntent;
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
pub fn install_source(
    st: &SystemTable<Boot>,
    kernel_data: &[u8],
    intent: BootIntent,
    evidence: [Module; 3],
) -> InstallHandoff {
    let loader = match load_file_from_esp(st, uefi::cstr16!("\\EFI\\BOOT\\BOOTX64.EFI")) {
        Ok(bytes) => region(Vec::leak(bytes), MODULE_KIND_LOADER_IMAGE),
        Err(_) => Module::default(),
    };
    let (store, mirror) = boot_disk(st);
    InstallHandoff {
        source: [loader, region(kernel_data, MODULE_KIND_KERNEL_IMAGE), boot_media(st)],
        evidence,
        store,
        mirror,
        requested: intent == BootIntent::Install,
        profile: 0,
    }
}

/* Leaked into loader memory like the loader file, so it outlives boot
 * services; a zero region when the firmware named no partition. */
fn boot_media(st: &SystemTable<Boot>) -> Module {
    match boot_partition(st.boot_services()) {
        Some(record) => Module {
            base: Box::leak(Box::new(record)) as *const _ as u64,
            size: BOOT_MEDIA_LEN as u64,
            kind: MODULE_KIND_BOOT_MEDIA,
            reserved: 0,
        },
        None => Module::default(),
    }
}

/* The package store, read here for a kernel whose own disk drivers may not
 * reach the disk it lies on, and from the same disk the live plan's model
 * files; a zero region for either the disk does not carry. */
fn boot_disk(st: &SystemTable<Boot>) -> (Module, Module) {
    let bs = st.boot_services();
    let Some(((base, size), disk)) = store_copy(bs) else {
        return (Module::default(), Module::default());
    };
    let store = Module { base, size, kind: MODULE_KIND_STORE, reserved: 0 };
    let mirror = match disk_mirror(bs, disk) {
        Some((base, size)) => Module { base, size, kind: MODULE_KIND_DISK_MIRROR, reserved: 0 },
        None => Module::default(),
    };
    (store, mirror)
}

fn region(bytes: &[u8], kind: u32) -> Module {
    Module { base: bytes.as_ptr() as u64, size: bytes.len() as u64, kind, reserved: 0 }
}
