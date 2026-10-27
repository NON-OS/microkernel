// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The firmware package: files a Wi-Fi card runs, which the tree does not
//! carry and no capsule or kernel image embeds. The image build places them
//! on the boot medium under `EFI/nonos/firmware/`, from a pinned
//! linux-firmware commit with their licence beside them; the loader reads
//! each into loader memory, which the kernel never reuses, and records it as
//! a module the kernel serves through `MkInstallSource` by kind. The loader
//! checks nothing about them: the driver that asks holds each file's length
//! and sha256 and refuses any other, so a file here is trusted no more than
//! the capsule that pins it allows. A file not on the medium is no module,
//! and the driver reports the package missing.

use alloc::vec::Vec;

use nonos_boot::handoff::types::Module;
use nonos_boot::loader::file::load_file_from_esp;
use uefi::prelude::*;
use uefi::CStr16;

/// The first file's module kind; the rest follow in `FILES`' order
/// (`src/syscall/microkernel/install_source_kinds.rs`).
pub const KIND_FIRMWARE_FIRST: u32 = 0x20;

/// MediaTek MT7921/MT7922 (`capsule_driver_mt7921/src/fw/pins.rs`): the
/// MT7961 patch and RAM code, the MT7961_1a pair (MT7920), the MT7922 pair.
pub const FILES: [&CStr16; 6] = [
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_MT7961_patch_mcu_1_2_hdr.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_RAM_CODE_MT7961_1.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_MT7961_patch_mcu_1a_2_hdr.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_RAM_CODE_MT7961_1a.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_MT7922_patch_mcu_1_1_hdr.bin"),
    uefi::cstr16!("\\EFI\\nonos\\firmware\\mediatek\\WIFI_RAM_CODE_MT7922_1.bin"),
];

/// No file in the package is larger; one that is is not read.
const MAX_FILE: usize = 4 << 20;

/// Each file found, as its module; one not found, or too large, has size 0.
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
