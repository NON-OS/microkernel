// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/amd_vi/regs.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/amd_vi/regs.rs"]
pub mod regs;

pub fn invalidate_all_supported(efr: u64) -> bool {
    regs::invalidate_all_supported(efr)
}

pub fn dev_table_base(phys: u64, pages: usize) -> u64 {
    regs::dev_table_base(phys, pages)
}

pub fn ring_base(phys: u64) -> u64 {
    regs::ring_base(phys)
}

pub fn ring_offset(index: u16) -> u64 {
    regs::ring_offset(index)
}

pub fn ring_index(register: u64) -> u16 {
    regs::ring_index(register)
}

pub fn ring_next(index: u16) -> u16 {
    regs::ring_next(index)
}

