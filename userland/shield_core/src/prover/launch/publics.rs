//! The proof's 37 public limbs as the pool's 13 words, inverting `PublicWords.publicsOf`: six
//! digests of four limbs, four scalars, two addresses of 48, 48, 48 and 16 bit limbs, and the
//! not-before time.

use crate::error::ProveError;

/// Limbs per intent, and words per intent on the pool.
pub const LIMBS: usize = 37;
pub const WORDS: usize = 13;
const MASK48: u128 = (1 << 48) - 1;

#[path = "publics_limbs.rs"]
mod publics_limbs;
pub use publics_limbs::address_limbs;

/// The pool's words, or PublicLimb for an address limb wider than its 48 or
/// 16 bits: written anyway, it would be a word that says something false, and
/// the pool would refuse the proof only after the person paid its gas.
pub fn pool_words(limbs: &[u64; LIMBS]) -> Result<[[u8; 32]; WORDS], ProveError> {
    let mut out = [[0u8; 32]; WORDS];
    let (_, rest) = limbs.split_at(24);
    let (scalars, rest) = rest.split_at(4);
    let (addresses, not_before) = rest.split_at(8);
    let (d, rest) = out.split_at_mut(6);
    let (s, rest) = rest.split_at_mut(4);
    let (a, nb) = rest.split_at_mut(2);
    d.copy_from_slice(&digest_words(limbs));
    s.iter_mut().zip(scalars).for_each(|(w, v)| scalar(w, Some(*v)));
    for (w, l) in a.iter_mut().zip(addresses.chunks_exact(4)) {
        address(w, l).ok_or(ProveError::PublicLimb)?;
    }
    nb.iter_mut().zip(not_before).for_each(|(w, v)| scalar(w, Some(*v)));
    Ok(out)
}

/// The six digest words alone: every limb of a digest is a whole u64, so
/// these always fit.
pub fn digest_words(limbs: &[u64; LIMBS]) -> [[u8; 32]; 6] {
    let mut out = [[0u8; 32]; 6];
    let digests = limbs.get(..24).unwrap_or_default();
    out.iter_mut().zip(digests.chunks_exact(4)).for_each(|(w, l)| digest(w, Some(l)));
    out
}

fn digest(word: &mut [u8; 32], limbs: Option<&[u64]>) {
    // Limb 0 is the lowest: the last eight bytes, big endian.
    for (chunk, v) in word.rchunks_exact_mut(8).zip(limbs.unwrap_or(&[])) {
        chunk.copy_from_slice(&v.to_be_bytes());
    }
}

fn scalar(word: &mut [u8; 32], v: Option<u64>) {
    word[24..].copy_from_slice(&v.unwrap_or(0).to_be_bytes());
}

/// None unless three limbs fit 48 bits and the top one 16.
fn address(word: &mut [u8; 32], limbs: &[u64]) -> Option<()> {
    let [l0, l1, l2, l3] = <[u64; 4]>::try_from(limbs).ok()?.map(u128::from);
    if l0 > MASK48 || l1 > MASK48 || l2 > MASK48 || l3 > 0xFFFF {
        return None;
    }
    let low = l0 | (l1 << 48) | ((l2 & 0xFFFF_FFFF) << 96);
    let high = u32::try_from((l2 >> 32) | (l3 << 16)).ok()?;
    word[12..16].copy_from_slice(&high.to_be_bytes());
    word[16..].copy_from_slice(&low.to_be_bytes());
    Some(())
}

#[cfg(test)]
#[path = "publics_test.rs"]
mod publics_test;
