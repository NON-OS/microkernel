// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/crypto/util/rng/entropy/health.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/crypto/util/rng/entropy/health.rs"]
pub mod health;

pub fn health_new() -> health::Health {
    health::Health::new()
}

pub fn health_proven(this: health::Health) -> bool {
    this.proven()
}

