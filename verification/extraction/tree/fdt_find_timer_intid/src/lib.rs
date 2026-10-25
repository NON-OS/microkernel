// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/fdt/irq/gic.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn gic_intid(cells: &[u8]) -> Result<u32, crate::arch::fdt::error::FdtError> {
    crate::arch::fdt::irq::gic_intid(cells)
}

