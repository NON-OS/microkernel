//! The two keys an address carries, in hex, as the address text holds them.

use nox_shield_core::ffi::AddressParts;
use nox_shield_core::keys::Address;

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
