// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/gic/distributor/device.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/gic/distributor/device.rs"]
pub mod device;

pub fn gicdistributor_new(base: u64) -> device::GicDistributor {
    device::GicDistributor::new(base)
}

