// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/cpu/cache_assoc.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/cpu/cache_assoc.rs"]
pub mod cache_assoc;

pub fn decode_l2_assoc(encoded: u8) -> u16 {
    cache_assoc::decode_l2_assoc(encoded)
}

