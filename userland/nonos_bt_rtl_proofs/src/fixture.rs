// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The capsule's embedded files, and what each ROM version must be sent,
//! written down from the file's own header as a hex dump reads it, not
//! from the parser under test: firmware version 0x75b8f098, two patches,
//! chip id 1 (ROM 0) at 0x30 for 0x4c14 octets and chip id 2 (ROM 1) at
//! 0x4cc0 for 0x8864.

use nonos_bt_hci::Host;

use crate::firmware::{CONFIG, FW};
use crate::model::Rtl;

pub const FW_VERSION: u32 = 0x75b8_f098;
/// (ROM version, offset, length) of each patch in the file.
pub const PATCHES: [(u8, usize, usize); 2] = [(0, 0x30, 0x4c14), (1, 0x4cc0, 0x8864)];
/// The HP's card's USB id.
pub const HP: (u16, u16) = (0x0bda, 0xb00a);

/// The image ROM `rom` must receive: its patch less four octets, the
/// firmware version, then the config.
pub fn expected(rom: u8) -> Vec<u8> {
    let (_, at, len) = PATCHES.iter().copied().find(|p| p.0 == rom).expect("a ROM with a patch");
    [&FW[at..at + len - 4], &FW_VERSION.to_le_bytes()[..], CONFIG].concat()
}

/// A host over a modelled ROM of version `rom` that accepts its image.
pub fn rom(rom: u8) -> Host<Rtl> {
    let accepts = if PATCHES.iter().any(|p| p.0 == rom) { expected(rom) } else { Vec::new() };
    Host::new(Rtl::new(rom, accepts))
}

/// The model behind a host.
pub fn chip(h: &mut Host<Rtl>) -> &mut Rtl {
    h.transport()
}
