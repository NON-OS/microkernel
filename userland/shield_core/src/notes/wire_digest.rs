//! A four-word digest as the pool's `bytes32`: the wallet's little endian words, byte reversed.
//! Pinned by the contract vector in `core/tests/vector.rs`. Packing it the other way commits to a
//! leaf the pool never stores.

/// The four-word digest packed as the pool reads it.
pub fn wire_digest(words: &[u64; 4]) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (chunk, word) in out.chunks_exact_mut(8).zip(words.iter()) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
    out.reverse();
    out
}
