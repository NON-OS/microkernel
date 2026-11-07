// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/iommu/smmu/encode/oas.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/iommu/smmu/encode/oas.rs"]
pub mod oas;

pub fn oas_bits(code: u8) -> u8 {
    oas::oas_bits(code)
}

