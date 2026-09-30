// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/iommu/regs/window.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn registers_fit(cap: u64, ecap: u64, window: usize) -> bool {
    crate::arch::x86_64::iommu::regs::window::registers_fit(cap, ecap, window)
}

