// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/regs/offsets/fault.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/regs/offsets/fault.rs"]
pub mod fault;

pub fn frcd_reason(high: u64) -> u8 {
    fault::frcd_reason(high)
}

pub fn frcd_source(high: u64) -> u16 {
    fault::frcd_source(high)
}

