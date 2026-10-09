//! A note's nullifier, the value the pool sees when the note is spent.
//! The leaf index is in the preimage so identical notes do not share a nullifier.
//! The derivation is the vendored circuit's, tied to it by `core/tests/nullifier.rs`.

use super::account::Account;
use crate::notes::wire_digest;
use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::field::Fp;
use stark_proofs::shield::key::nullifier;
use stark_proofs::shield::note::POOL_LOG_ROUNDS;

fn nullifier_with(nk: [Fp; RATE], cm: &[u64; 4], leaf_index: u64) -> [Fp; RATE] {
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let cm_fp: [Fp; RATE] =
        [Fp::from_u64(cm[0]), Fp::from_u64(cm[1]), Fp::from_u64(cm[2]), Fp::from_u64(cm[3])];
    nullifier(&h, nk, cm_fp, leaf_index, true)
}

/// The same nullifier as the pool's `bytes32`, for matching a `NullifierSpent` log.
pub fn note_nullifier_wire(account: &Account, cm: &[u64; 4], leaf_index: u64) -> [u8; 32] {
    nullifier_wire_with(account.nk(), cm, leaf_index)
}

/// The same from `nk` alone, which a full view key carries.
pub fn nullifier_wire_with(nk: [Fp; RATE], cm: &[u64; 4], leaf_index: u64) -> [u8; 32] {
    let nf = nullifier_with(nk, cm, leaf_index);
    wire_digest(&[nf[0].value(), nf[1].value(), nf[2].value(), nf[3].value()])
}
