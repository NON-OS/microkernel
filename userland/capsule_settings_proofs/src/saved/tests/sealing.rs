// NONOS Operating System (AGPL-3.0-or-later)

//! Sealing the record: its key, its refusals, and the WPA3 pin kept.

use super::super::error::SavedError;
use super::super::file::{open_file, seal_file, DIR, FILE_LEN, PATH};
use super::super::list::SavedList;
use super::fixture::{two, KEY, NONCE};

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
fn every_refusal_names_itself_and_the_record_lives_in_its_directory() {
    use SavedError::*;
    let all = [NoStore, NotKept, NoTpm, BootChanged, KeyFailed, Unreadable, Damaged, NotSavable];
    let all = [all.as_slice(), &[Vfs("vfs")]].concat();
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
