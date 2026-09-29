// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/regs/offsets/invalidate.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/regs/offsets/invalidate.rs"]
pub mod invalidate;

pub fn iva_offset(ecap: u64) -> usize {
    invalidate::iva_offset(ecap)
}

pub fn iotlb_offset(ecap: u64) -> usize {
    invalidate::iotlb_offset(ecap)
}

