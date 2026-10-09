//! The randomness one launch proof consumes: the created notes' secrets and
//! blindings, and the proof's own blinding, all drawn from these bytes.
//!
//! Drawn from the platform CSPRNG, used once, and wiped when dropped. A second
//! use in the same process is refused, because the same bytes would make the
//! same blinding, and two proofs under one blinding reveal the witness.

use crate::entropy::{entropy_was_used, fill};
use crate::error::ProveError;
use zeroize::Zeroizing;

/// Bytes `nox_prover` takes for one proof.
pub const ENTROPY_BYTES: usize = nox_prover::ENTROPY_BYTES;

/// Single use randomness for one spend.
pub struct SpendEntropy {
    bytes: Zeroizing<[u8; ENTROPY_BYTES]>,
}

impl SpendEntropy {
    pub fn draw() -> Result<SpendEntropy, ProveError> {
        let mut bytes = Zeroizing::new([0u8; ENTROPY_BYTES]);
        fill(bytes.as_mut()).map_err(|_| ProveError::SeedEntropy)?;
        Ok(SpendEntropy { bytes })
    }

    /// Spend the bytes. Refused if these exact bytes were spent before.
    pub fn consume(&self) -> Result<&[u8], ProveError> {
        if entropy_was_used(self.bytes.as_ref()) {
            return Err(ProveError::SeedReused);
        }
        Ok(self.bytes.as_ref())
    }
}
