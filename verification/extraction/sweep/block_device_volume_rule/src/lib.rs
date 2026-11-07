// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/block_device/volume_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/block_device/volume_rule.rs"]
pub mod volume_rule;

pub fn admits(volume_io: bool, driver: bool, storage: bool) -> bool {
    volume_rule::admits(volume_io, driver, storage)
}

