// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/security/mte/mode.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/security/mte/mode.rs"]
pub mod mode;

pub fn mtemode_tcf(this: mode::MteMode) -> u64 {
    this.tcf()
}

