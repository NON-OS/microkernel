// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The firmware package: files a Wi-Fi card runs, never embedded in a capsule
//! or the kernel. The image build places them under `EFI/nonos/firmware/`; the
//! loader reads each into loader memory and records it as a module of kind
//! 0x20 + its index in `FILES` (`src/syscall/microkernel/install_source_kinds.rs`),
//! which the kernel serves through `MkInstallSource`. The loader checks nothing:
//! the driver holds each file's length and sha256 and refuses any other
//! (`capsule_driver_mt7921/src/fw/pins.rs`: the MT7961, MT7961_1a and MT7922
//! patch and RAM code). A file not on the medium, or over 4 MiB, is no module.

use alloc::vec::Vec;

use nonos_boot::handoff::types::Module;
use nonos_boot::loader::file::load_file_from_esp;
use uefi::prelude::*;
use uefi::CStr16;

pub const KIND_FIRMWARE_FIRST: u32 = 0x20;

pub const FILES: [&CStr16; 6] = [
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_MT7961_patch_mcu_1_2_hdr.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_RAM_CODE_MT7961_1.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_MT7961_patch_mcu_1a_2_hdr.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_RAM_CODE_MT7961_1a.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_MT7922_patch_mcu_1_1_hdr.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_RAM_CODE_MT7922_1.bin"),
];

const MAX_FILE: usize = 4 << 20;

pub fn firmware_package(st: &SystemTable<Boot>) -> [Module; 6] {
    let mut out = [Module::default(); 6];
    for (i, path) in FILES.iter().enumerate() {
        let Ok(bytes) = load_file_from_esp(st, path) else { continue };
        if bytes.is_empty() || bytes.len() > MAX_FILE {
            continue;
        }
        let bytes: &'static [u8] = Vec::leak(bytes);
        out[i] = Module {
            base: bytes.as_ptr() as u64,
            size: bytes.len() as u64,
            kind: KIND_FIRMWARE_FIRST + i as u32,
            reserved: 0,
        };
    }
    out
}
