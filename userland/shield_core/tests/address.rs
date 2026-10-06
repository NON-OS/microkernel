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

use nox_shield_core::ffi::AddressParts;
use nox_shield_core::keys::Address;

fn sample() -> Address {
    Address {
        spend_pk: [1, 0xFFFF_FFFF_0000_0000, 42, 7],
        view_pk: [
            9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ],
    }
}

#[test]
fn an_address_round_trips_through_its_text() {
    let a = sample();
    let parsed = Address::parse(&a.to_text()).expect("our own encoding parses");
    assert_eq!(a, parsed);
}

#[test]
fn case_and_whitespace_are_forgiven() {
    let text = sample().to_text();
    let messy = format!("  {}  ", text.to_uppercase());
    assert_eq!(Address::parse(&messy).expect("a pasted address parses"), sample());
}

#[test]
fn a_flipped_character_is_refused() {
    let text = sample().to_text();
    let mut bytes = text.into_bytes();
    // The prefix is four characters, so this lands in the payload.
    bytes[10] = if bytes[10] == b'a' { b'b' } else { b'a' };
    let flipped = String::from_utf8(bytes).expect("still ascii");
    assert!(Address::parse(&flipped).is_err(), "a mistyped address must not resolve");
}

#[test]
fn a_truncated_address_is_refused() {
    let text = sample().to_text();
    assert!(Address::parse(&text[..text.len() - 3]).is_err());
}

#[test]
fn a_foreign_prefix_is_refused() {
    let text = sample().to_text();
    let swapped = text.replacen("nox1", "bc1q", 1);
    assert!(Address::parse(&swapped).is_err(), "an address from elsewhere must not parse");
}

#[test]
fn a_character_outside_the_alphabet_is_refused() {
    let text = sample().to_text();
    let mut bytes = text.into_bytes();
    bytes[8] = b'1';
    let bad = String::from_utf8(bytes).expect("still ascii");
    assert!(Address::parse(&bad).is_err());
}

// The two keys an address carries, in hex, must match the address text. A printed spend key
// the address does not commit to invites a payment elsewhere. Byte order fails silently, so
// the words are spelled out here, not read back through the same helper.
#[test]
fn the_parts_are_the_keys_the_address_carries() {
    let address = Address { spend_pk: [1, 2, 3, 4], view_pk: [0xab; 32] };
    let parts = AddressParts::of(&address);
    assert_eq!(parts.spend_hex, "0100000000000000020000000000000003000000000000000400000000000000");
    assert_eq!(parts.view_hex, "ab".repeat(32));
}

#[test]
fn a_parsed_address_has_the_parts_of_the_one_that_was_written() {
    let address = Address { spend_pk: [9, 8, 7, 6], view_pk: [0x11; 32] };
    let read = Address::parse(&address.to_text()).expect("round trip");
    assert_eq!(AddressParts::of(&read), AddressParts::of(&address));
}
