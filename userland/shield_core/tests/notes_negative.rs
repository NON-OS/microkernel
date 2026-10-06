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
    commitment, commitment_bytes, fresh_blinding, open_note, seal_note, NoteCipher, NotePlaintext,
    Opened, CIPHER_LEN,
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
fn a_tampered_ciphertext_does_not_open() {
    let mine = account();
    let note = plain(&mine);
    let cm = commitment_bytes(&commitment(&pool_hasher(), &note.note()));
    let mut sealed = seal_note(&note, &mine.address(), &cm).expect("seal");
    sealed.sealed[3] ^= 1;
    assert!(matches!(open_note(&sealed, mine.view(), &cm), Opened::AuthFail));
}

#[test]
fn a_ciphertext_does_not_open_against_another_commitment() {
    let mine = account();
    let note = plain(&mine);
    let cm = commitment_bytes(&commitment(&pool_hasher(), &note.note()));
    let sealed = seal_note(&note, &mine.address(), &cm).expect("seal");
    let mut other = cm;
    other[0] ^= 1;
    assert!(matches!(open_note(&sealed, mine.view(), &other), Opened::AuthFail));
}

#[test]
fn a_malformed_record_is_refused_before_the_aead() {
    assert!(NoteCipher::decode(&[0u8; CIPHER_LEN - 1]).is_none());
    assert!(NoteCipher::decode(&[0u8; CIPHER_LEN + 1]).is_none());
    assert!(NoteCipher::decode(&[0u8; CIPHER_LEN]).is_some());
}
