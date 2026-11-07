// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/regs/cap/extended.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/regs/cap/extended.rs"]
pub mod extended;

pub fn snoop_control(ecap: u64) -> bool {
    extended::snoop_control(ecap)
}

pub fn queued_invalidation(ecap: u64) -> bool {
    extended::queued_invalidation(ecap)
}

pub fn interrupt_remapping(ecap: u64) -> bool {
    extended::interrupt_remapping(ecap)
}

pub fn extended_interrupt_mode(ecap: u64) -> bool {
    extended::extended_interrupt_mode(ecap)
}

