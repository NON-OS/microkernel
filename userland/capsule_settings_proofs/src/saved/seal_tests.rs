// NONOS Operating System (AGPL-3.0-or-later)

//! A sealed record opens under its key only, and keeps a WPA3 network off
//! WPA2 through sealing.

use super::error::SavedError;
use super::file::{open_file, seal_file, FILE_LEN};
use super::saved_rig::*;

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
