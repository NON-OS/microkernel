// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/crypto/util/rng/entropy/credit.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/crypto/util/rng/entropy/credit.rs"]
pub mod credit;

pub fn enough(cpu: usize, capsule: usize) -> bool {
    credit::enough(cpu, capsule)
}

