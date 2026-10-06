//! The seeds already spent in this process, held as digests so the record holds no secret.

use blake3::Hasher;
use nonos_stark::air::RATE;
use nonos_stark::field::Fp;
use std::collections::HashSet;
use std::sync::Mutex;
use std::sync::OnceLock;

fn used() -> &'static Mutex<HashSet<[u8; 32]>> {
    static USED: OnceLock<Mutex<HashSet<[u8; 32]>>> = OnceLock::new();
    USED.get_or_init(|| Mutex::new(HashSet::new()))
}

/// Record a seed as used and report whether it was seen before. A poisoned lock counts as
/// used: refusing a proof beats allowing one whose freshness nothing tracks.
pub fn seed_was_used(seed: &[Fp; RATE]) -> bool {
    let mut h = Hasher::new();
    h.update(b"nox-shield/blinding-seed/v1");
    for e in seed.iter() {
        h.update(&e.value().to_le_bytes());
    }
    let digest: [u8; 32] = *h.finalize().as_bytes();
    match used().lock() {
        Ok(mut set) => !set.insert(digest),
        Err(_) => true,
    }
}

/// Whether these bytes already went to the launch prover. They seed note secrets, blindings
/// and the proof blinding, so a repeat reuses all three.
pub fn entropy_was_used(bytes: &[u8]) -> bool {
    let digest: [u8; 32] =
        *blake3::keyed_hash(blake3::hash(b"nox-shield/launch-entropy/v1").as_bytes(), bytes)
            .as_bytes();
    match used().lock() {
        Ok(mut set) => !set.insert(digest),
        Err(_) => true,
    }
}
