//! What a spent seed leaves behind, which is nothing. The wipe is volatile because an optimiser
//! removes a plain store to a value about to drop, and Debug never prints the seed.

use super::seed::ProofSeed;
use nonos_stark::field::Fp;

impl Drop for ProofSeed {
    fn drop(&mut self) {
        for w in self.words.iter_mut() {
            // SAFETY: same as the draw buffer, the write must not be elided.
            unsafe { core::ptr::write_volatile(w, Fp::ZERO) };
        }
    }
}

impl core::fmt::Debug for ProofSeed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("ProofSeed(redacted)")
    }
}
