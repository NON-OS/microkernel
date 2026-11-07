// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/block_device/gpt/array.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/block_device/gpt/array.rs"]
pub mod array;

pub fn array_sectors(entries: u32, size: u32) -> u64 {
    array::array_sectors(entries, size)
}

pub fn shaped(entries: u32, size: u32) -> bool {
    array::shaped(entries, size)
}

pub fn within(lba: u64, entries: u32, size: u32, lo: u64, hi: u64) -> bool {
    array::within(lba, entries, size, lo, hi)
}

