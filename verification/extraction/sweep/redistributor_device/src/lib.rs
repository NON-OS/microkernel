// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/gic/redistributor/device.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/gic/redistributor/device.rs"]
pub mod device;

pub fn gicredistributor_new(base: u64) -> device::GicRedistributor {
    device::GicRedistributor::new(base)
}

