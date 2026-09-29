// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/fpu/context.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/fpu/context.rs"]
pub mod context;

pub fn fpsimdcontext_zeroed() -> context::FpSimdContext {
    context::FpSimdContext::zeroed()
}

