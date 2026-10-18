// NONOS Operating System (AGPL-3.0-or-later)

//! The list's slots: replacing, dropping, and lengths decode refuses.

use super::super::file::{is_withdrawn, FILE_LEN};
use super::super::list::{SavedList, PLAIN_LEN, SLOTS};
use super::fixture::two;

#[test]
fn put_replaces_by_name_and_drops_the_oldest_when_full() {
    let mut l = two();
    assert!(l.put(b"home", b"new passphrase"));
    assert_eq!((l.len(), l.find(b"home")), (2, Some(1)));
    assert_eq!(l.passphrase(1), b"new passphrase");
    for n in 0..SLOTS as u8 {
        assert!(l.put(&[b'n', n], b"12345678"));
    }
    assert_eq!((l.len(), l.find(b"cafe"), l.find(b"home")), (SLOTS, None, None));
    assert!(!l.put(b"", b"x") && !l.put(&[b'a'; 33], b"") && !l.put(b"a", &[b'p'; 65]));
    l.remove(0);
    assert_eq!((l.len(), l.ssid(0)), (SLOTS - 1, &[b'n', 1][..]));
}

#[test]
fn decode_refuses_lengths_no_slot_was_written_with() {
    let mut plain = [0u8; PLAIN_LEN];
    two().encode(&mut plain);
    assert_eq!(SavedList::decode(&plain).map(|l| l.len()), Some(2));
    plain[0] = 33;
    assert!(SavedList::decode(&plain).is_none());
    assert!(SavedList::decode(&plain[1..]).is_none());
    assert!(is_withdrawn(&[0u8; FILE_LEN]) && !is_withdrawn(&[1u8]));
}
