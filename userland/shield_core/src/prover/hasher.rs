//! The pool hash, in a single place.

use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::field::Fp;
use stark_proofs::shield::note::POOL_LOG_ROUNDS;

/// The pool hash. Its round count must match every in circuit compression, or commitments go
/// unrecognised and membership proves a permutation the hash never ran, both looking like a bad
/// proof.
pub fn pool_hasher() -> Poseidon {
    Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE])
}
