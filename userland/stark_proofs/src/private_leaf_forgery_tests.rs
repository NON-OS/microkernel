// NONOS Operating System (AGPL-3.0-or-later)
//! T2's counterexample, kept as a test so it cannot quietly move: the forgery
//! the public-leaf gate refuses verifies under the private-leaf gate.

use crate::crypto::stark::air::{build_attestation_trailer_from_set, verify_membership_trailer};
use crate::crypto::stark::air::{verify_public_trailer, Poseidon, RATE};
use crate::crypto::stark::attest_params::{EXTRA_BLOWUP_BITS, GRIND_BITS, LOG_ROUNDS, N_QUERIES};
use crate::crypto::stark::field::Fp;
use crate::public_leaf_tests::{context, root_bytes, set, DEPTH, ROGUE};

#[test]
fn the_private_leaf_gate_accepts_the_forgery() {
    let s = set(false);
    let ctx = context(ROGUE, 7);
    let h = Poseidon::new(LOG_ROUNDS, [Fp::ZERO; RATE]);
    let forged = build_attestation_trailer_from_set(
        &h,
        LOG_ROUNDS,
        &s,
        2,
        &ctx,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    );
    let root = root_bytes(s.root());
    let ok = verify_membership_trailer(
        &h,
        LOG_ROUNDS,
        root,
        DEPTH,
        &forged,
        &ctx,
        N_QUERIES,
        GRIND_BITS,
        EXTRA_BLOWUP_BITS,
    );
    assert!(ok, "the private-leaf forgery stopped verifying; T2's counterexample moved");
    assert!(!verify_public_trailer(&root, DEPTH, ROGUE, &forged, &ctx), "old magic accepted");
}
