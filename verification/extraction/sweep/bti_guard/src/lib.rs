// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/security/bti/guard.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/security/bti/guard.rs"]
pub mod guard;

pub fn btiguard_instruction(this: guard::BtiGuard) -> u32 {
    this.instruction()
}

