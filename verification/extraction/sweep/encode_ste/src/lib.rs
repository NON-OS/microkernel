// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/iommu/smmu/encode/ste.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/iommu/smmu/encode/ste.rs"]
pub mod ste;

pub fn l1_descriptor(l2_phys: u64, split: u8) -> u64 {
    ste::l1_descriptor(l2_phys, split)
}

