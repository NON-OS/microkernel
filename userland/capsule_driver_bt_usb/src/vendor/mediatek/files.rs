/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The package's Bluetooth files as `MkInstallSource` serves them. They
//! are never built into this capsule or the kernel: MediaTek's licence
//! forbids a form the GPL or LGPL would cover. The image carries them in
//! `EFI/nonos/firmware/mediatek/` beside the Wi-Fi driver's, the loader
//! records each one present as a module, and the kernel serves it by kind.

use super::chip::Chip;
use super::pins::{hash, PINS};

/// One file of the package.
pub struct File {
    /// Its `MkInstallSource` kind: the loader's module kind for it.
    pub kind: u64,
    pub len: usize,
    pub sha256: [u8; 32],
}

/// The first Bluetooth file's kind, after the Wi-Fi driver's 0x20..0x25;
/// slot `i` is `KIND_FIRST + i`.
pub const KIND_FIRST: u64 = 0x26;

const fn file(slot: usize) -> File {
    let (_, len, hex) = PINS[slot];
    File { kind: KIND_FIRST + slot as u64, len, sha256: hash(hex) }
}

pub const FILES: [File; 2] = [file(0), file(1)];

/// The patch a chip takes, from `files` (`FILES` in the capsule).
pub fn for_chip(files: &[File; 2], chip: Chip) -> &File {
    match chip {
        Chip::Mt7961 => &files[0],
        Chip::Mt7922 => &files[1],
    }
}
