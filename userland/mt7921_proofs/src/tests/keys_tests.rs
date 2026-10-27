// NONOS Operating System (AGPL-3.0-or-later)

//! The key commands' bytes against `mt76_connac_mcu_sta_key_tlv` and
//! `struct sta_rec_sec`; `PROTECT_FRAME` in the TXD; and RXD1 read for what
//! the chip did, as `mt7921_mac_fill_rx` reads it.

use crate::mac::rxd::parse;
use crate::mac::sec::{judge, restore, Sec};
use crate::mac::txd::{txwi, TxSpec};
use crate::model::rxd::rx_buffer_with;
use crate::vif::key::{group, pairwise};

#[test]
fn the_pairwise_key_goes_under_the_access_points_entry() {
    let b = pairwise(&[7; 16]);
    assert_eq!(b.len(), 8 + 80, "the request header, then sta_rec_sec put whole");
    assert_eq!(b[..8], [0, 1, 1, 0, 1, 0, 0, 0], "BSS 0, wcid 1, one TLV, appended, MUAR 0");
    assert_eq!(b[8..14], [0x11, 0, 44, 0, 0, 1], "KEY_V2, cut to one key, SET_KEY, one");
    assert_eq!(b[16..20], [5, 36, 0, 16], "CCMP, sec_key's size, index 0, 16 octets");
    assert_eq!(b[20..36], [7; 16]);
    assert!(b[36..].iter().all(|&x| x == 0), "nothing past the key");
}

#[test]
fn the_group_key_and_the_igtk_go_under_the_own_entry() {
    let b = group(2, &[8; 16], Some(&[9; 16]));
    assert_eq!(b[..8], [0, 19, 1, 0, 1, 0xe, 0, 0], "wcid 19, MUAR 0xe");
    assert_eq!(b[8..14], [0x11, 0, 80, 0, 0, 2]);
    assert_eq!((&b[16..20], &b[52..56]), (&[5, 36, 2, 16][..], &[10, 36, 0, 16][..]), "CCMP, BIP");
    assert_eq!((&b[20..36], &b[56..72]), (&[8; 16][..], &[9; 16][..]));
    assert_eq!(group(1, &[8; 16], None)[8..14], [0x11, 0, 44, 0, 0, 1], "no IGTK: one key");
}

#[test]
fn protect_frame_is_set_only_for_a_frame_the_chip_seals() {
    let f = [0x08, 0x41, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2];
    let s = |protect| TxSpec { wcid: 1, token: 1, frame_dev: 0, rate: 0, protect };
    let w3 = |t: [u8; 64]| u32::from_le_bytes(t[12..16].try_into().unwrap());
    assert_eq!(w3(txwi(&f, &s(true)).unwrap()) & 2, 2);
    assert_eq!(w3(txwi(&f, &s(false)).unwrap()) & 2, 0);
}

#[test]
fn rxd1_says_decrypted_only_for_ccmp_without_a_mismatch() {
    assert_eq!(judge(4 << 16 | 2 << 21, true), Sec::Decrypted(2));
    assert_eq!(judge(4 << 16, false), Sec::Plain, "not Protected: a BIP frame keeps its MMIE");
    assert_eq!(judge(0, true), Sec::Plain, "no cipher: as it came");
    assert_eq!(
        (judge(4 << 16 | 1 << 23, true), judge(4 << 16 | 1 << 24, true)),
        (Sec::Plain, Sec::Plain)
    );
    assert_eq!(judge(2 << 16, true), Sec::Refused, "TKIP is never installed here");
}

#[test]
fn a_decrypted_frame_comes_back_with_its_ccmp_header() {
    let mut f = vec![0x08, 0x42, 0, 0];
    f.extend_from_slice(&[0; 20]);
    f.extend_from_slice(b"plain");
    let b = rx_buffer_with(&f, -40, 4 << 16 | 1 << 21 | 1 << 11, 0x0605_0403_0201);
    let r = parse(&b, 1).unwrap();
    assert_eq!(r.sec, Some((0x0605_0403_0201, 1)), "group 1's packet number, key index 1");
    let mut out = [0u8; 64];
    let n = restore(&b[r.start..r.start + r.len], 24, 0x0605_0403_0201, 1, &mut out).unwrap();
    assert_eq!(out[24..32], [1, 2, 0, 0x60, 3, 4, 5, 6], "PN0, PN1, ExtIV with index 1, PN2..5");
    assert_eq!((&out[32..37], &out[37..n]), (&b"plain"[..], &[0u8; 8][..]));
    let no_g1 = rx_buffer_with(&f, -40, 4 << 16, 0);
    assert!(parse(&no_g1, 1).is_none(), "decrypted with no packet number to check");
}
