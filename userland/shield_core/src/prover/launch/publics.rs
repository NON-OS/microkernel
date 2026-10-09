//! The proof's 37 public limbs as the pool's 13 words, inverting `PublicWords.publicsOf`: six
//! digests of four limbs, four scalars, two addresses of 48, 48, 48 and 16 bit limbs, and the
//! not-before time.

/// Limbs per intent, and words per intent on the pool.
pub const LIMBS: usize = 37;
pub const WORDS: usize = 13;
const MASK48: u128 = (1 << 48) - 1;

/// An address as the statement's four limbs, lowest first.
pub fn address_limbs(addr: &[u8; 20]) -> [u64; 4] {
    let (high, low) = split(addr);
    let limb = |v: u128| u64::try_from(v & MASK48).unwrap_or(0);
    [
        limb(low),
        limb(low >> 48),
        limb((low >> 96) | (u128::from(high & 0xFFFF) << 32)),
        u64::from(high >> 16),
    ]
}

pub fn pool_words(limbs: &[u64; LIMBS]) -> [[u8; 32]; WORDS] {
    let mut out = [[0u8; 32]; WORDS];
    let (digests, rest) = limbs.split_at(24);
    let (scalars, rest) = rest.split_at(4);
    let (addresses, not_before) = rest.split_at(8);
    let (d, rest) = out.split_at_mut(6);
    let (s, rest) = rest.split_at_mut(4);
    let (a, nb) = rest.split_at_mut(2);
    d.iter_mut().zip(digests.chunks_exact(4)).for_each(|(w, l)| digest(w, Some(l)));
    s.iter_mut().zip(scalars).for_each(|(w, v)| scalar(w, Some(*v)));
    a.iter_mut().zip(addresses.chunks_exact(4)).for_each(|(w, l)| address(w, Some(l)));
    nb.iter_mut().zip(not_before).for_each(|(w, v)| scalar(w, Some(*v)));
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

fn address(word: &mut [u8; 32], limbs: Option<&[u64]>) {
    let l = |i: usize| u128::from(limbs.and_then(|s| s.get(i)).copied().unwrap_or(0));
    let low = l(0) | (l(1) << 48) | ((l(2) & 0xFFFF_FFFF) << 96);
    let high = (l(2) >> 32) | (l(3) << 16);
    word[12..16].copy_from_slice(&u32::try_from(high).unwrap_or(0).to_be_bytes());
    word[16..].copy_from_slice(&low.to_be_bytes());
}

/// An address as its top 32 bits and low 128 bits.
fn split(addr: &[u8; 20]) -> (u32, u128) {
    let mut hi = [0u8; 4];
    let mut lo = [0u8; 16];
    hi.copy_from_slice(&addr[..4]);
    lo.copy_from_slice(&addr[4..]);
    (u32::from_be_bytes(hi), u128::from_be_bytes(lo))
}

#[cfg(test)]
#[path = "publics_test.rs"]
mod publics_test;
