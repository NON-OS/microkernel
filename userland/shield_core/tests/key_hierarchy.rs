//! The key hierarchy against the prover's published oracle.
//!
//! `spec/shield-key-hierarchy.json` in the STARKs repository gives, for fixed
//! secrets, every value the wallet derives: the spend key, the nullifier key,
//! the owner digest, the commitment, and the live and dead nullifiers. A wallet
//! that disagrees on any of them holds notes it cannot spend or cannot find.

use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::field::Fp;
use nox_shield_core::notes::commitment;
use nox_shield_core::prover::pool_hasher;
use stark_proofs::shield::key::{derive, nullifier};
use stark_proofs::shield::note::{owner_commit, Note, POOL_LOG_ROUNDS};

const ORACLE: &str = include_str!("data/shield-key-hierarchy.json");

fn words(v: &serde_like::Value) -> [u64; 4] {
    let a = v.as_array();
    [a[0], a[1], a[2], a[3]]
}

fn fp(w: [u64; 4]) -> [Fp; RATE] {
    [Fp::from_u64(w[0]), Fp::from_u64(w[1]), Fp::from_u64(w[2]), Fp::from_u64(w[3])]
}

fn val(e: [Fp; RATE]) -> [u64; 4] {
    [e[0].value(), e[1].value(), e[2].value(), e[3].value()]
}

#[test]
fn every_case_in_the_oracle_derives_the_same() {
    let cases = serde_like::cases(ORACLE);
    assert_eq!(cases.len(), 3, "the oracle's three cases");
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    for c in &cases {
        let keys = derive(&h, fp(words(&c["sk"])));
        assert_eq!(val(keys.spend_pk), words(&c["spend_pk"]), "spend_pk");
        assert_eq!(val(keys.nk), words(&c["nk"]), "nk");
        let note = Note {
            value: c["value"].as_u64(),
            asset_id: c["asset_id"].as_u64(),
            spend_pk: words(&c["spend_pk"]),
            blinding: words(&c["blinding"]),
        };
        assert_eq!(val(owner_commit(&note)), words(&c["owner"]), "owner");
        let cm = commitment(&pool_hasher(), &note);
        assert_eq!(val(cm), words(&c["cm"]), "the wallet's own commitment");
        let leaf = c["leaf_index"].as_u64();
        assert_eq!(val(nullifier(&h, keys.nk, cm, leaf, true)), words(&c["nf"]), "nf");
        assert_eq!(val(nullifier(&h, keys.nk, cm, leaf, false)), words(&c["nf_dead"]), "dead nf");
    }
}

/// The oracle is flat JSON of integers and integer arrays, and this reads only that.
#[path = "key_hierarchy/serde_like.rs"]
mod serde_like;
