// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/firmware_flash.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/firmware_flash.rs"]
pub mod firmware_flash;

pub fn is_flash_function(segment: u16, bus: u8, device: u8, function: u8) -> bool {
    firmware_flash::is_flash_function(segment, bus, device, function)
}

pub fn overlaps_flash(base: u64, size: u64) -> bool {
    firmware_flash::overlaps_flash(base, size)
}

