//! A transfer and withdrawals the prover's request checks take.

use super::super::Destination;
use super::{accepted, accepted_of, account, held, held_of, FINNEY, MILLI};
use crate::net::asset::ETH;

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
fn an_eth_withdrawal_is_one_the_prover_takes() {
    let me = account();
    let notes = [held_of(&me, ETH, 5 * FINNEY, 1)];
    accepted_of(&me, ETH, &notes, Destination::Withdraw { recipient: [7; 20] }, FINNEY);
}
