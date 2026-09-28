// NONOS Operating System (AGPL-3.0-or-later)
//! Base58 as Nym spells its keys: one spelling per 32-byte key.

use crate::directory_sync::api::base58::decode32;

#[test]
fn live_keys_decode() {
    // Node identities from the validator's exit-gateway list, 2026-09-28.
    for k in include_str!("../vectors/exits.txt").lines() {
        assert!(decode32(k.trim().as_bytes()).is_some(), "{k}");
    }
}

#[test]
fn empty_text_is_not_the_zero_key() {
    assert_eq!(decode32(b""), None);
}

#[test]
fn the_zero_key_is_thirty_two_ones() {
    assert_eq!(decode32(&[b'1'; 32]), Some([0u8; 32]));
    assert_eq!(decode32(&[b'1'; 31]), None);
    assert_eq!(decode32(&[b'1'; 33]), None);
}

#[test]
fn leading_ones_must_match_leading_zero_bytes() {
    let k = b"CXcCVGiamYSwgVwaxW3mEkXkZh1sKY2TXnWjjTjxDxzA";
    let mut padded = b"1".to_vec();
    padded.extend_from_slice(k);
    assert!(decode32(k).is_some());
    assert_eq!(decode32(&padded), None, "a second spelling of the same key");
    assert_eq!(decode32(b"2"), None, "a number too small to fill 32 bytes");
}

#[test]
fn characters_outside_the_alphabet_are_refused() {
    for bad in [&b"0abc"[..], b"Oabc", b"Iabc", b"labc", b"ab c"] {
        assert_eq!(decode32(bad), None);
    }
}
