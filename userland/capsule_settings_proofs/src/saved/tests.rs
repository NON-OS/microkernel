/*
 * NONOS Operating System (AGPL-3.0-or-later)
 */

//! The record's slots, encoding and sealing, as shipped.

use super::error::SavedError;
use super::file::{is_withdrawn, open_file, seal_file, DIR, FILE_LEN, PATH};
use super::list::{SavedList, PLAIN_LEN, SLOTS};

const KEY: [u8; 32] = [7; 32];
const NONCE: [u8; 12] = [9; 12];

fn two() -> SavedList {
    let mut l = SavedList::new();
    assert!(l.put(b"home", b"correct horse"));
    assert!(l.put(b"cafe", b""));
    l
}

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
fn a_sealed_record_opens_under_its_key_only() {
    let mut file = [0u8; FILE_LEN];
    assert!(seal_file(&two(), &KEY, &NONCE, &mut file));
    assert!(!file.windows(13).any(|w| w == b"correct horse"));
    let back = open_file(&file, &KEY).ok().unwrap();
    assert_eq!(
        (back.len(), back.ssid(0), back.passphrase(0)),
        (2, &b"home"[..], &b"correct horse"[..])
    );
    assert!(matches!(open_file(&file, &[8; 32]), Err(SavedError::Unreadable)));
    let mut bent = file;
    bent[FILE_LEN - 1] ^= 1;
    assert!(matches!(open_file(&bent, &KEY), Err(SavedError::Unreadable)));
    bent = file;
    bent[0] = b'X';
    assert!(matches!(open_file(&bent, &KEY), Err(SavedError::Damaged)));
    assert!(matches!(open_file(&file[..FILE_LEN - 1], &KEY), Err(SavedError::Damaged)));
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

#[test]
fn a_network_saved_as_wpa3_stays_so_through_sealing() {
    use crate::join_wire::JoinFlags;
    let wpa3 = JoinFlags { wpa3_only: true, hidden: false };
    let mut l = two();
    assert!(l.put_with(b"wpa3net", b"sae password", wpa3));
    let mut file = [0u8; FILE_LEN];
    assert!(seal_file(&l, &KEY, &NONCE, &mut file));
    let back = open_file(&file, &KEY).ok().unwrap();
    let i = back.find(b"wpa3net").unwrap();
    assert_eq!(back.join_flags(i), wpa3, "the flag is sealed with the passphrase");
    assert_eq!(back.join_flags(back.find(b"home").unwrap()), JoinFlags::default());
}
