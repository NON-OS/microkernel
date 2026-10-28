// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The firmware package: files a Wi-Fi card or a Bluetooth controller runs,
//! never embedded in a capsule or the kernel. The image build places them under
//! `EFI/nonos/firmware/`; the loader reads each into loader memory and records it
//! as a module of kind 0x20 + its index in `FILES`, which the kernel serves
//! through `MkInstallSource` (`src/syscall/microkernel/install_source_kinds.rs`).
//! The loader checks nothing: each driver holds its files' lengths and sha256s
//! (`capsule_driver_mt7921/src/fw/pins.rs`, kinds 0x20..0x25, and
//! `capsule_driver_bt_usb/src/vendor/mediatek/pins.rs`, 0x26 and 0x27). A file
//! not on the medium, or over 4 MiB, is no module; one not on the medium is
//! passed over without a word.

use alloc::vec::Vec;

use nonos_boot::handoff::types::Module;
use nonos_boot::loader::file::{file_exists, load_file_from_esp};
use uefi::prelude::*;
use uefi::CStr16;

pub const KIND_FIRMWARE_FIRST: u32 = 0x20;

pub const FILES: [&CStr16; 8] = [
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_MT7961_patch_mcu_1_2_hdr.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_RAM_CODE_MT7961_1.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_MT7961_patch_mcu_1a_2_hdr.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_RAM_CODE_MT7961_1a.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_MT7922_patch_mcu_1_1_hdr.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_RAM_CODE_MT7922_1.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\BT_RAM_CODE_MT7961_1_2_hdr.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\BT_RAM_CODE_MT7922_1_1_hdr.bin"),
];

const MAX_FILE: usize = 4 << 20;

pub fn firmware_package(st: &SystemTable<Boot>) -> [Module; 8] {
    let mut out = [Module::default(); 8];
    for (i, path) in FILES.iter().enumerate() {
        if !file_exists(st, path) {
            continue;
        }
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
