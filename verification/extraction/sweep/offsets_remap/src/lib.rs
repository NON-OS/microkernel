// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/regs/offsets/remap.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/regs/offsets/remap.rs"]
pub mod remap;

pub fn irta_value(table_phys: u64, x2apic: bool) -> u64 {
    remap::irta_value(table_phys, x2apic)
}

pub fn irta_entries(irta: u64) -> u32 {
    remap::irta_entries(irta)
}

pub fn gcmd_strict(persistent: u32, bit: u32) -> u32 {
    remap::gcmd_strict(persistent, bit)
}

pub fn is_strict(status: u32) -> bool {
    remap::is_strict(status)
}

