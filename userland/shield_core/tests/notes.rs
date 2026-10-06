/*
 * A test asserts by panicking, and a measurement prints what it read, so the
 * lints that forbid panicking and indexing are off here and nowhere else.
 */
#![allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use nox_shield_core::custody::{generate_phrase, phrase_to_seed};
use nox_shield_core::keys::Account;
use nox_shield_core::notes::{
    commitment, commitment_bytes, fresh_blinding, open_note, seal_note, NotePlaintext, Opened,
};
use nox_shield_core::prover::pool_hasher;

fn account() -> Account {
    let phrase = generate_phrase().expect("entropy");
    Account::from_seed(&phrase_to_seed(&phrase).expect("seed")).expect("account")
}

fn plain(to: &Account) -> NotePlaintext {
    let pk = to.address().spend_pk;
    NotePlaintext {
        value: 1234,
        asset_id: 0,
        blinding: fresh_blinding().expect("entropy"),
        spend_pk: pk,
    }
}

#[test]
fn a_note_seals_to_its_recipient_and_opens_for_nobody_else() {
    let mine = account();
    let other = account();
    let note = plain(&mine);
    let cm = commitment_bytes(&commitment(&pool_hasher(), &note.note()));
    let sealed = seal_note(&note, &mine.address(), &cm).expect("seal");

    match open_note(&sealed, mine.view(), &cm) {
        Opened::Note(opened) => {
            assert_eq!(opened.value, note.value);
            assert_eq!(opened.blinding, note.blinding);
            assert_eq!(opened.spend_pk, note.spend_pk);
        }
        _ => panic!("a note sealed to this account did not open"),
    }
    assert!(
        matches!(open_note(&sealed, other.view(), &cm), Opened::TagMiss),
        "another account's viewing key must not reach the ciphertext"
    );
}
