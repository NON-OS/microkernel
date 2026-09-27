// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/uefi/crc.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/uefi/crc.rs"]
pub mod crc;

pub fn crc32_new() -> crc::Crc32 {
    crc::Crc32::new()
}

pub fn crc32_finalize(this: crc::Crc32) -> u32 {
    this.finalize()
}

pub fn crc32_current(this: crc::Crc32) -> u32 {
    this.current()
}

