use super::receiving_address::{
    parse_receiving_address, receiving_address, receiving_address_text,
};
use super::Account;
use crate::custody::{generate_phrase, phrase_to_seed, Phrase};
use crate::notes::{
    commitment, open_xwing, seal_xwing, wire_digest, NotePlaintext, Opening, XwingOpened,
};
use crate::prover::pool_hasher;
use sha3::{Digest, Sha3_256};
use x_wing::{DecapsulationKey, Decapsulator, KeyExport};
#[path = "receiving_address_text_test.rs"]
mod text;

fn account(phrase: &Phrase) -> Account {
    Account::from_seed(&phrase_to_seed(phrase).expect("seed")).expect("account")
}

#[test]
fn the_key_section_is_the_provers_pinned_encapsulation_key() {
    // Payee seed 0x07 x 32 in the prover's published vector: its encapsulation key
    // hashes to 4c8260..., and the address must carry those bytes.
    let dk = DecapsulationKey::from([0x07u8; 32]);
    let mut ek = [0u8; 1216];
    ek.copy_from_slice(&dk.encapsulation_key().to_bytes());
    let a = receiving_address(&[1, 2, 3, 4], &ek);
    assert_eq!((a.len(), a[0]), (1249, 0x01));
    assert_eq!(&a[1..9], &1u64.to_le_bytes(), "spend key words little endian");
    let h: [u8; 32] = Sha3_256::digest(&a[33..]).into();
    let hex: String = h.iter().map(|x| format!("{x:02x}")).collect();
    assert_eq!(hex, "4c82604001ad0b7a4cbc022c322f5e757f09ba4a5b095c8af545caeeac6836c8");
}

#[test]
fn restoring_the_words_restores_the_address() {
    let phrase = generate_phrase().expect("entropy");
    assert_eq!(account(&phrase).receiving_address(), account(&phrase).receiving_address());
    let other = generate_phrase().expect("entropy");
    assert_ne!(account(&phrase).receiving_address(), account(&other).receiving_address());
}

#[test]
fn a_sender_with_only_the_text_can_pay_this_wallet() {
    let me = account(&generate_phrase().expect("entropy"));
    let text = receiving_address_text(&me.receiving_address());
    assert!(text.starts_with("nox1"));
    // The sender parses the text, builds a note to the spend key, seals it to
    // the X-Wing key, and the leaf is that note's commitment.
    let (spend_pk, ek) = parse_receiving_address(&text).expect("parses");
    let o = Opening { value: 7, asset_id: 1, blinding: [9, 8, 7, 6] };
    let plain =
        NotePlaintext { value: o.value, asset_id: o.asset_id, blinding: o.blinding, spend_pk };
    let cm = commitment(&pool_hasher(), &plain.note());
    let leaf = wire_digest(&[cm[0].value(), cm[1].value(), cm[2].value(), cm[3].value()]);
    let ek = x_wing::EncapsulationKey::try_from(ek.as_slice()).expect("ek");
    let blob = seal_xwing(&o, &ek, &leaf).expect("seal");
    // This wallet opens it with its own receiving key and its own spend key.
    match open_xwing(&blob, me.receive().dk(), &leaf, &me.address().spend_pk) {
        XwingOpened::Note(got) => assert_eq!(got.value, 7),
        _ => panic!("the wallet could not open a note paid to its own address"),
    }
}
