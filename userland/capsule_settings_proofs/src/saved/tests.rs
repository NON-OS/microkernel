// NONOS Operating System (AGPL-3.0-or-later)

//! The saved list's slots and encoding: a name replaced in place, the
//! oldest dropped when full, and every malformed length refused.

use super::error::SavedError;
use super::file::{is_withdrawn, DIR, FILE_LEN, PATH};
use super::list::{SavedList, PLAIN_LEN, SLOTS};
use super::saved_rig::*;

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

#[test]
fn every_refusal_names_itself_and_the_record_lives_in_its_directory() {
    use SavedError::*;
    let all = [NoStore, NotKept, NoTpm, BootChanged, KeyFailed, Unreadable, Damaged, Vfs("vfs")];
    for (i, a) in all.iter().enumerate() {
        assert!(all[i + 1..].iter().all(|b| a.text() != b.text()));
    }
    assert!(PATH.starts_with(DIR) && SavedList::new().is_empty() && !two().is_empty());
}
