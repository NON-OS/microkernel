//! The per proof blinding seed, drawn once and spent once. Two proofs of one statement under
//! one seed subtract to the witness, so a reused seed is refused process wide, in `used`.

use super::fill::fill;
use super::used::seed_was_used;
use crate::error::ProveError;
use nonos_stark::air::{seed_from_entropy, RATE};
use nonos_stark::field::Fp;

/// Bytes drawn per seed, with margin for words at or above the modulus being rejected.
const DRAW: usize = 96;

/// A single use blinding seed from the platform CSPRNG, zeroed on drop.
pub struct ProofSeed {
    pub(super) words: [Fp; RATE],
}

impl ProofSeed {
    /// Draw a fresh seed. Every hidden proof calls this, and nothing memoises it.
    pub fn draw() -> Result<ProofSeed, ProveError> {
        let mut bytes = [0u8; DRAW];
        fill(&mut bytes).map_err(|_| ProveError::SeedEntropy)?;
        let words = seed_from_entropy(&bytes).ok_or(ProveError::SeedEntropy);
        for b in bytes.iter_mut() {
            // SAFETY: a volatile write is not elided, so the draw leaves no stack copy.
            unsafe { core::ptr::write_volatile(b, 0) };
        }
        Ok(ProofSeed { words: words? })
    }

    /// Rebuild a seed from given bytes, for a known answer test. Production draws.
    pub fn from_bytes(bytes: &[u8]) -> Result<ProofSeed, ProveError> {
        let words = seed_from_entropy(bytes).ok_or(ProveError::SeedEntropy)?;
        Ok(ProofSeed { words })
    }

    /// Hand the seed to the prover. A seed already spent in this process is refused.
    pub fn consume(self) -> Result<[Fp; RATE], ProveError> {
        if seed_was_used(&self.words) {
            return Err(ProveError::SeedReused);
        }
        Ok(self.words)
    }
}
