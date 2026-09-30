// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/security/bti/pad.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/security/bti/pad.rs"]
pub mod pad;

pub fn is_bti_landing_pad(instruction: u32) -> bool {
    pad::is_bti_landing_pad(instruction)
}

