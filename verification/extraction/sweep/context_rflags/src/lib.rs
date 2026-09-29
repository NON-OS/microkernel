// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/context/rflags.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/context/rflags.rs"]
pub mod context_rflags;

pub fn sanitize(rflags: u64) -> u64 {
    context_rflags::sanitize(rflags)
}

pub fn sanitize_user(rflags: u64) -> u64 {
    context_rflags::sanitize_user(rflags)
}

