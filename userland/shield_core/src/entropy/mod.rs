//! Randomness, under one rule: every hidden proof gets a fresh blinding seed from the platform
//! CSPRNG, and a seed is never used twice.

mod fill;
mod redact;
mod seed;
mod used;

pub use fill::fill;
pub use seed::ProofSeed;
pub use used::{entropy_was_used, seed_was_used};
