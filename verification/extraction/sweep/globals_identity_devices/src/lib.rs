// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/globals/identity_devices.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/globals/identity_devices.rs"]
pub mod identity_devices;

pub fn identity_devices() -> usize {
    identity_devices::identity_devices()
}

