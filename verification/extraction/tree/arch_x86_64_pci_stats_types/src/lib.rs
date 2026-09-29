// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/pci/stats_types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn pcistats_new() -> crate::arch::x86_64::pci::stats_types::PciStats {
    crate::arch::x86_64::pci::stats_types::PciStats::new()
}

