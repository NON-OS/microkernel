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

use alloc::{boxed::Box, vec::Vec};

use nonos_boot::handoff::types::{
    InstallHandoff, Module, BOOT_MEDIA_LEN, MODULE_KIND_BOOT_MEDIA, MODULE_KIND_KERNEL_IMAGE,
    MODULE_KIND_LOADER_IMAGE,
};
use nonos_boot::loader::file::{boot_partition, load_file_from_esp};
use nonos_boot::menu::BootIntent;
use uefi::prelude::*;

use super::boot_disk::boot_disk;

pub fn install_source(
    st: &SystemTable<Boot>,
    kernel_data: &[u8],
    intent: BootIntent,
    evidence: [Module; 3],
) -> InstallHandoff {
    let loader = load_file_from_esp(st, super::loader_path::LOADER_PATH)
        .map_or(Module::default(), |b| region(Vec::leak(b), MODULE_KIND_LOADER_IMAGE));
    let (store, mirror) = boot_disk(st);
    InstallHandoff {
        source: [loader, region(kernel_data, MODULE_KIND_KERNEL_IMAGE), boot_media(st)],
        evidence,
        store,
        mirror,
        firmware: super::firmware_package::firmware_package(st),
        requested: intent == BootIntent::Install,
        profile: 0,
    }
}

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

fn region(bytes: &[u8], kind: u32) -> Module {
    Module { base: bytes.as_ptr() as u64, size: bytes.len() as u64, kind, reserved: 0 }
}
