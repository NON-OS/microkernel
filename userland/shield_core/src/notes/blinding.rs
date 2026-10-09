//! A fresh blinding for every note, from the platform CSPRNG, never a counter or other fields.
//! Without it two notes of one value to one recipient would share a commitment.

use crate::entropy::fill;
use crate::error::CustodyError;
use nonos_stark::air::{seed_from_entropy, RATE};

/// A fresh note blinding: four canonical field words from the platform CSPRNG.
/// A word at or above the modulus would reduce to another blinding's commitment and strand the
/// note, so the prover's own rejection sampler draws them.
pub fn fresh_blinding() -> Result<[u64; 4], CustodyError> {
    let mut bytes = [0u8; 96];
    fill(&mut bytes)?;
    let words = seed_from_entropy(&bytes).ok_or(CustodyError::Entropy)?;
    let mut out = [0u64; 4];
    for (slot, w) in out.iter_mut().zip(words.iter().take(RATE)) {
        *slot = w.value();
    }
    Ok(out)
}
