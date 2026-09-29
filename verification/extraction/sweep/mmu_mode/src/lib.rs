// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/riscv64/mmu/mode.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/riscv64/mmu/mode.rs"]
pub mod mode;

pub fn mmumode_satp_mode(this: mode::MmuMode) -> Option<usize> {
    this.satp_mode()
}

pub fn mmumode_va_bits(this: mode::MmuMode) -> usize {
    this.va_bits()
}

pub fn mmumode_levels(this: mode::MmuMode) -> usize {
    this.levels()
}

