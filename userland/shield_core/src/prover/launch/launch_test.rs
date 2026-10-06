/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic, clippy::indexing_slicing)]

//! The request and seed the wallet writes, read by the prover's own parser:
//! the trees rebuild to the named roots, the notes sit at the named leaves,
//! the secrets own them, and the values balance. Nothing is proved here, so this runs in a
//! moment. The proof itself is the ignored test beside it.

use super::fixture::transfer;
use super::seed::seed_json;
use crate::custody::{generate_phrase, phrase_to_seed};
use crate::keys::Account;
use nonos_stark::field::Fp;

fn account() -> Account {
    Account::from_seed(&phrase_to_seed(&generate_phrase().expect("entropy")).expect("seed"))
        .expect("account")
}

#[test]
fn the_prover_accepts_what_the_wallet_writes() {
    let me = account();
    let (request, a, b) = transfer(&me);
    let secret = me.sk().map(|f| f.value());
    let seed = seed_json(&secret, [&a, &b]);
    let mut n = 0u64;
    let mut words = |k: usize| {
        Ok((0..k)
            .map(|_| {
                n += 1;
                Fp::from_u64(n)
            })
            .collect())
    };
    let built = stark_proofs::host::build_parts_with(&request.to_json(), &seed, &mut words);
    let (_, created) = built.expect("the prover refused the request");
    assert_eq!(created.notes[0].value, 1_000_000);
    assert_eq!(created.notes[1].spend_pk, me.address().spend_pk, "change is keyed to this wallet");
    assert_eq!(created.owned, [false, false], "both outputs keyed by name, neither to a fresh key");
}

#[test]
fn a_root_the_leaves_do_not_reach_is_refused_before_proving() {
    let me = account();
    let (mut request, a, b) = transfer(&me);
    request.note_root[0] ^= 1;
    let seed = seed_json(&me.sk().map(|f| f.value()), [&a, &b]);
    let mut words = |k: usize| Ok(vec![Fp::from_u64(1); k]);
    let refused = stark_proofs::host::build_parts_with(&request.to_json(), &seed, &mut words);
    assert!(refused.is_err());
}
