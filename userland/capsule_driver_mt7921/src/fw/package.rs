/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The firmware package: MediaTek's files, which are never built into this
//! capsule or the kernel (their licence forbids a form the GPL would cover,
//! and NONOS is AGPL). The image carries them as a separate artefact in
//! `EFI/nonos/firmware/mediatek/` with `LICENCE.mediatek` beside them; the
//! loader reads each one present into memory and the kernel serves it by
//! its slot through `MkInstallSource`. A file is accepted only at the
//! length and SHA-256 `pins` names (Linux v6.12's names, `mt792x.h`).

use super::pins::{unhex, PINS};
use crate::ids::Chip;

/// One file of the package.
pub struct File {
    pub name: &'static str,
    /// Its `MkInstallSource` kind: the loader's module kind for it.
    pub kind: u64,
    pub len: usize,
    pub sha256: [u8; 32],
}

/// The first slot's kind; slot `i` is `SLOT_FIRST + i`.
pub const SLOT_FIRST: u64 = 0x20;

const fn file(slot: usize) -> File {
    let (name, len, hex) = PINS[slot];
    File { name, kind: SLOT_FIRST + slot as u64, len, sha256: unhex(hex) }
}

pub const FILES: [File; 6] = [file(0), file(1), file(2), file(3), file(4), file(5)];

/// The ROM patch and RAM code a chip runs (`mt792x_patch_name`,
/// `mt792x_ram_name`).
pub fn for_chip(chip: Chip) -> (&'static File, &'static File) {
    let at = match chip {
        Chip::Mt7961 => 0,
        Chip::Mt7920 => 2,
        Chip::Mt7922 => 4,
    };
    (&FILES[at], &FILES[at + 1])
}
