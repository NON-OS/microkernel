/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

//! A transfer and a withdrawal as the wallet builds them, read by the prover's
//! own request checks: roots, membership, the dummy beside a single note,
//! balance and ownership. Nothing is proved, so this runs in a moment.

use super::build::request;
use super::pick::pick;
use super::{Destination, Order};
use crate::custody::{generate_phrase, phrase_to_seed};
use crate::keys::Account;
use crate::net::asset::{Asset, ETH, NOX};
use crate::net::pool::LAUNCH;
use crate::notes::{commitment, NotePlaintext, NoteRecord, NoteStatus};
use crate::prover::launch::seed::seed_json;
use crate::prover::launch::tree::pool_root;
use crate::prover::pool_hasher;
use crate::wallet::anchor::Anchor;
use nonos_stark::field::Fp;

const MILLI: u64 = 1_000_000;

fn account() -> Account {
    Account::from_seed(&phrase_to_seed(&generate_phrase().expect("entropy")).expect("seed"))
        .expect("account")
}

/// 0.001 ETH in wei, the smallest standard ETH size.
const FINNEY: u64 = 1_000_000_000_000_000;

fn held(me: &Account, value: u64, leaf: u64) -> NoteRecord {
    held_of(me, NOX, value, leaf)
}

fn held_of(me: &Account, asset: Asset, value: u64, leaf: u64) -> NoteRecord {
    let plain = NotePlaintext {
        value,
        asset_id: asset.id,
        blinding: [leaf + 5, 6, 7, 8],
        spend_pk: me.address().spend_pk,
    };
    let cm = commitment(&pool_hasher(), &plain.note()).map(|f| f.value());
    NoteRecord { plain, leaf_index: leaf, cm, status: NoteStatus::Unspent, found_at: leaf }
}

fn accepted(me: &Account, notes: &[NoteRecord], to: Destination, amount: u64) {
    accepted_of(me, NOX, notes, to, amount);
}

fn accepted_of(me: &Account, asset: Asset, notes: &[NoteRecord], to: Destination, amount: u64) {
    let mut leaves: Vec<[u64; 4]> = vec![[1, 2, 3, 4]];
    leaves.extend(notes.iter().map(|n| n.cm));
    let anchor = Anchor { root: pool_root(&leaves), leaves };
    let refs: Vec<&NoteRecord> = notes.iter().collect();
    let order =
        Order { asset, to, amount, fee: 1_000, fee_to: [9; 20], not_before: 600, early: false };
    let picked =
        pick(&refs, &anchor, asset.id, amount + 1_000, &|_| true).expect("notes that cover it");
    let req = request(&LAUNCH, &anchor, &picked, &order, me.address().spend_pk)
        .expect("the pool's rules");
    let seed = seed_json(&me.sk().map(|f| f.value()), [&picked.notes[0], &picked.notes[1]]);
    let mut words = |k: usize| Ok(vec![Fp::from_u64(3); k]);
    stark_proofs::host::build_parts_with(&req.to_json(), &seed, &mut words)
        .expect("the prover refused it");
}

#[test]
fn a_transfer_from_two_notes_is_one_the_prover_takes() {
    let (me, payee) = (account(), account());
    let notes = [held(&me, 2 * MILLI, 1), held(&me, 2 * MILLI, 2)];
    let to = Destination::Wallet {
        spend_pk: payee.address().spend_pk,
        sealed_to: Box::new(payee.receive().encapsulation_key()),
    };
    accepted(&me, &notes, to, 2 * MILLI);
}

#[test]
fn a_withdrawal_from_a_single_note_takes_a_dummy_beside_it() {
    let me = account();
    accepted(&me, &[held(&me, 5 * MILLI, 1)], Destination::Withdraw { recipient: [7; 20] }, MILLI);
}

#[test]
fn a_payment_of_an_odd_size_is_refused_before_the_prover_sees_it() {
    let me = account();
    let notes = [held(&me, 5 * MILLI, 1)];
    let refs: Vec<&NoteRecord> = notes.iter().collect();
    let leaves = vec![[1, 2, 3, 4], notes[0].cm];
    let anchor = Anchor { root: pool_root(&leaves), leaves };
    let order = Order {
        asset: NOX,
        to: Destination::Withdraw { recipient: [7; 20] },
        amount: 3 * MILLI,
        fee: 1_000,
        fee_to: [9; 20],
        not_before: 600,
        early: false,
    };
    let picked =
        pick(&refs, &anchor, 1, 3 * MILLI + 1_000, &|_| true).expect("notes that cover it");
    assert!(request(&LAUNCH, &anchor, &picked, &order, me.address().spend_pk).is_err());
}

#[test]
fn an_eth_withdrawal_is_one_the_prover_takes() {
    let me = account();
    let notes = [held_of(&me, ETH, 5 * FINNEY, 1)];
    accepted_of(&me, ETH, &notes, Destination::Withdraw { recipient: [7; 20] }, FINNEY);
}

#[test]
fn an_eth_note_is_never_picked_to_pay_nox() {
    let me = account();
    let notes = [held_of(&me, ETH, 5 * FINNEY, 1)];
    let refs: Vec<&NoteRecord> = notes.iter().collect();
    let leaves = vec![[1, 2, 3, 4], notes[0].cm];
    let anchor = Anchor { root: pool_root(&leaves), leaves };
    assert!(pick(&refs, &anchor, NOX.id, MILLI, &|_| true).is_none());
}
