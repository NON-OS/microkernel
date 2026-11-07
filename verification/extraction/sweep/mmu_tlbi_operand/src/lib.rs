// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/mmu/tlbi_operand.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/mmu/tlbi_operand.rs"]
pub mod tlbi_operand;

pub fn vaae1is(va: u64) -> u64 {
    tlbi_operand::vaae1is(va)
}

