// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/capabilities/resource/nonce_compose.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/capabilities/resource/nonce_compose.rs"]
pub mod nonce_compose;

pub fn compose(timestamp: u64, counter: u64) -> u64 {
    nonce_compose::compose(timestamp, counter)
}

