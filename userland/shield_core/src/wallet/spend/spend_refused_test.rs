//! A payment of an odd size, and an ETH note to pay NOX, refused before the prover.

use super::super::build::request;
use super::super::pick::pick;
use super::super::{Destination, Order};
use super::{account, held, held_of, FINNEY, MILLI};
use crate::net::asset::{ETH, NOX};
use crate::net::pool::LAUNCH;
use crate::notes::NoteRecord;
use crate::prover::launch::tree::pool_root;
use crate::wallet::anchor::Anchor;

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
fn an_eth_note_is_never_picked_to_pay_nox() {
    let me = account();
    let notes = [held_of(&me, ETH, 5 * FINNEY, 1)];
    let refs: Vec<&NoteRecord> = notes.iter().collect();
    let leaves = vec![[1, 2, 3, 4], notes[0].cm];
    let anchor = Anchor { root: pool_root(&leaves), leaves };
    assert!(pick(&refs, &anchor, NOX.id, MILLI, &|_| true).is_none());
}
