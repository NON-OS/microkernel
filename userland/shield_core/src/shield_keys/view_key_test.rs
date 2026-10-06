/*
 * A test asserts by panicking, so the lints that forbid it are off here.
 */
#![allow(clippy::unwrap_used, clippy::indexing_slicing)]

//! A view key reads back to the keys it was made from, and a changed symbol or a relabelled
//! prefix is refused.

use super::accounts_test::seed;
use super::view_key_read::parse_view_key;
use super::view_key_text::view_key_text;
use super::{Account, ViewKind};

#[test]
fn view_keys_read_back_and_refuse_a_changed_symbol() {
    let account = Account::at(&seed(), 1).unwrap();
    for kind in [ViewKind::Incoming, ViewKind::Full] {
        let text = view_key_text(&account, kind);
        let read = parse_view_key(&text).unwrap();
        assert_eq!(read.kind, kind);
        assert_eq!(read.receive.encapsulation_key(), account.receive().encapsulation_key());
        assert_eq!(read.spend_pk, account.spend_pk().map(|f| f.value()));
        assert_eq!(read.nk, (kind == ViewKind::Full).then(|| account.nk().map(|f| f.value())));
        let mut bad = text.to_string();
        let last = bad.pop().unwrap();
        bad.push(if last == 'a' { 'b' } else { 'a' });
        assert!(parse_view_key(&bad).is_err());
        let other = if kind == ViewKind::Full { "noxivk1" } else { "noxfvk1" };
        assert!(parse_view_key(&format!("{other}{}", &text[7..])).is_err());
    }
}

/// BLAKE3 of `noxivk1` and `noxfvk1` of accounts 0, 1 and 2. The texts are in docs/09-accounts.md.
const VIEW_VECTORS: [[&str; 2]; 3] = [
    [
        "eb845b176a0b5bd8852e7b49e371735460e0a978808e36889e95c80367a680ac",
        "e7c288bd9269b6f9e0cb7456a19d7ec38abedb5366e02490dcc5ef416f125e2e",
    ],
    [
        "4a29a1b804ca23b01b02ee0ed071055df0ac1fc32ec7c3868b8e2fd5184f857f",
        "31dcc7ed6796143063bc06bedf3ad72f8b8e61e0a62895d5988c254f11569e4f",
    ],
    [
        "82ee6cc4ef462b628ede8b692f88e8fe245db149ae407541a6c63da0b60cdc0f",
        "66c1939cd8cf369b08f4344fa7691470ce3599a76127196c857e38867decba3c",
    ],
];

#[test]
fn view_keys_of_accounts_zero_to_two_match_the_vectors() {
    for (i, expected) in (0u32..).zip(VIEW_VECTORS.iter()) {
        let account = Account::at(&seed(), i).unwrap();
        for (kind, want) in [ViewKind::Incoming, ViewKind::Full].into_iter().zip(expected) {
            let text = view_key_text(&account, kind);
            assert_eq!(
                blake3::hash(text.as_bytes()).to_hex().as_str(),
                *want,
                "account {i} {kind:?}"
            );
        }
    }
}
