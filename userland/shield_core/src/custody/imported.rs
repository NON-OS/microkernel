//! A wallet from a private key, with no words behind it. The key is the public account, and the
//! shield keys come from a 64-byte value derived one way from the key under its own label, so no
//! shield key leads back to the key and no other wallet derives the same shield.

use super::secret::Seed;
use crate::error::CustodyError;
use zeroize::{Zeroize, Zeroizing};

/// The label the shield of an imported key is derived under.
const CONTEXT: &str = "nox-shield 2026 imported key seed v1";

/// The value the shield keys of `key` derive from, in the place a phrase's seed would be.
pub fn seed_of_key(key: &[u8; 32]) -> Seed {
    let mut h = blake3::Hasher::new_derive_key(CONTEXT);
    h.update(key);
    let mut out = [0u8; 64];
    h.finalize_xof().fill(&mut out);
    let seed = Seed::new(out);
    out.zeroize();
    seed
}

/// Read a private key of 64 hex digits, `0x` optional, refused unless it is a key on the curve.
pub fn parse_key(text: &str) -> Result<Zeroizing<[u8; 32]>, CustodyError> {
    let digits = text.trim().trim_start_matches("0x");
    if digits.len() != 64 {
        return Err(CustodyError::KeyShape);
    }
    let mut key = Zeroizing::new([0u8; 32]);
    for (slot, pair) in key.iter_mut().zip(digits.as_bytes().chunks_exact(2)) {
        let pair = std::str::from_utf8(pair).map_err(|_| CustodyError::KeyShape)?;
        *slot = u8::from_str_radix(pair, 16).map_err(|_| CustodyError::KeyShape)?;
    }
    k256::ecdsa::SigningKey::from_bytes(key.as_ref().into()).map_err(|_| CustodyError::KeyShape)?;
    Ok(key)
}
