// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/regs/cap/fault.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/regs/cap/fault.rs"]
pub mod fault;

pub fn fault_recording_offset(cap: u64) -> usize {
    fault::fault_recording_offset(cap)
}

pub fn fault_recording_count(cap: u64) -> u16 {
    fault::fault_recording_count(cap)
}

