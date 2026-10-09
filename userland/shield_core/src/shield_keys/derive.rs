//! Turning the seed into keys with BLAKE3 key derivation under a context string.
//! Field draws use the prover's rejection sampler, since a reduced hash would bias the spend key.

use crate::error::CustodyError;
use blake3::Hasher;
use nonos_stark::air::{seed_from_entropy, RATE};
use nonos_stark::field::Fp;
use zeroize::Zeroize;

/// Bytes drawn before rejection sampling, a wide margin over the 32 strictly required.
const DRAW: usize = 128;

/// Derive `RATE` uniform field elements from an account's key material under a context string.
pub fn field_key(seed: &[u8], context: &str) -> Result<[Fp; RATE], CustodyError> {
    let mut extract = Hasher::new_derive_key(context);
    extract.update(seed);
    let mut bytes = [0u8; DRAW];
    extract.finalize_xof().fill(&mut bytes);
    let words = seed_from_entropy(&bytes);
    bytes.zeroize();
    words.ok_or(CustodyError::Entropy)
}

/// Derive 32 bytes from the seed under a context string, for keys off the field.
pub fn byte_key(seed: &[u8], context: &str) -> [u8; 32] {
    let mut extract = Hasher::new_derive_key(context);
    extract.update(seed);
    *extract.finalize().as_bytes()
}

/// The note store's row key.
pub fn store_key(seed: &[u8]) -> [u8; 32] {
    byte_key(seed, super::domain::STORE_CONTEXT)
}
