// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/diag/cpl.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/diag/cpl.rs"]
pub mod cpl;

pub fn cpl_from_cs(cs: u64) -> u8 {
    cpl::cpl_from_cs(cs)
}

