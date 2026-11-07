// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/hypervisor/hv/regs.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/hypervisor/hv/regs.rs"]
pub mod regs;

pub fn page(gpa: u64) -> u64 {
    regs::page(gpa)
}

pub fn sint(vector: u8, masked: bool) -> u64 {
    regs::sint(vector, masked)
}

pub fn scontrol_enabled() -> u64 {
    regs::scontrol_enabled()
}

