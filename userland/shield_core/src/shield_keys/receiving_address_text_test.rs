//! The address as text: a damaged or old one refused, the pinned one and the phone's read.

use super::super::receiving_address::{
    parse_receiving_address, receiving_address, receiving_address_text,
};
use super::account;
use crate::custody::generate_phrase;
use x_wing::{DecapsulationKey, Decapsulator, KeyExport};

#[test]
fn a_damaged_or_old_address_is_refused() {
    let me = account(&generate_phrase().expect("entropy"));
    let text = receiving_address_text(&me.receiving_address());
    let mut chars: Vec<char> = text.chars().collect();
    let i = chars.len() / 2;
    chars[i] = if chars[i] == 'a' { 'b' } else { 'a' };
    let flipped: String = chars.into_iter().collect();
    assert!(parse_receiving_address(&flipped).is_err(), "one character changed");
    // The old two-key address, 113 characters, is a different thing entirely.
    assert!(parse_receiving_address(&me.address().to_text()).is_err());
}

#[test]
fn the_text_is_the_one_the_nonos_wallet_checks() {
    // The NONOS wallet refuses a payee before asking the shield service unless it
    // is "nox1" and 2,005 letters of a to z and 2 to 7. The pinned payee's text is
    // kept beside the wallet's proofs, which run that check on it.
    let dk = DecapsulationKey::from([0x07u8; 32]);
    let mut ek = [0u8; 1216];
    ek.copy_from_slice(&dk.encapsulation_key().to_bytes());
    let text = receiving_address_text(&receiving_address(&[1, 2, 3, 4], &ek));
    assert_eq!(text.len(), 2009);
    let pinned = include_str!("../../../wallet_proofs/fixtures/nox1-pinned.txt");
    assert_eq!(pinned, text);
}

#[test]
fn an_address_the_phone_app_made_parses() {
    // Made by the NONOS shield app on an iPhone (0.9.0, Sepolia): the same
    // checksum, version, spend key words and X-Wing key are read here.
    let phone = include_str!("../../../wallet_proofs/fixtures/nox1-phone.txt");
    let (spend_pk, ek) = parse_receiving_address(phone).expect("the phone's address parses");
    assert!(spend_pk.iter().all(|w| *w < nonos_stark::field::P));
    assert_eq!(ek.len(), 1216);
}
