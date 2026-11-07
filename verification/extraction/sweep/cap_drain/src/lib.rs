// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/regs/cap/drain.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/regs/cap/drain.rs"]
pub mod drain;

pub fn read_drain(cap: u64) -> bool {
    drain::read_drain(cap)
}

pub fn write_drain(cap: u64) -> bool {
    drain::write_drain(cap)
}

