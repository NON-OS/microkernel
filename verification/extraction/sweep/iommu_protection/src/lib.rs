// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/iommu/protection.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/iommu/protection.rs"]
pub mod protection;

pub fn iommuprotection_readable(this: protection::IommuProtection) -> bool {
    this.readable()
}

pub fn iommuprotection_writable(this: protection::IommuProtection) -> bool {
    this.writable()
}

