// NONOS Operating System (AGPL-3.0-or-later)

//! A received frame's descriptor, walked as `mt7921_mac_fill_rx` walks
//! it: the groups the second word announces, in their order and sizes,
//! the header offset's pad, the best chain's signal; and the refusals:
//! errors, band 1, a translated header. What the chip decrypted is
//! `sec_tests`'.

use crate::mac::rxd::{parse, RxFrame};
use crate::mac::{kind, Kind};
use crate::model::rxd::rx_buffer;

fn with(d1: u32, d2: u32, groups: usize, frame_len: usize) -> Vec<u8> {
    let mut b = vec![0u8; 24 + 4 * groups + frame_len];
    let len = b.len() as u32;
    b[0..4].copy_from_slice(&(len | 2 << 27).to_le_bytes());
    b[4..8].copy_from_slice(&d1.to_le_bytes());
    b[8..12].copy_from_slice(&d2.to_le_bytes());
    b
}

#[test]
fn the_models_frame_is_found_after_group_3_with_its_signal() {
    let b = rx_buffer(&[0x80u8; 40], -61);
    assert_eq!(parse(&b, 0x3), Some(RxFrame { start: 32, len: 40, rssi: -61, sec: None }));
    assert_eq!(kind(&b), Kind::Frame);
}

#[test]
fn every_group_and_the_pad_move_the_start() {
    let all = 1 << 11 | 1 << 12 | 1 << 13 | 1 << 14 | 1 << 15;
    let b = with(all, 2 << 14, 4 + 4 + 2 + 2 + 18 + 1, 40);
    assert_eq!(parse(&b, 0x3).map(|f| f.start), Some(24 + 4 * 30 + 4), "30 words, then 2x2 pad");
    let g5_alone = with(1 << 15, 0, 0, 40);
    assert_eq!(parse(&g5_alone, 0x3).map(|f| f.start), Some(24), "group 5 only with group 3");
    assert!(parse(&with(1 << 14, 0, 2, 0), 0x3).is_none(), "groups past the buffer");
}

#[test]
fn errors_band_1_and_translation_are_refused() {
    for d1 in [1 << 25, 1 << 26, 1 << 27, 1 << 28] {
        assert!(parse(&with(d1, 0, 0, 40), 0x3).is_none(), "{d1:#x}");
    }
    for d2 in [1 << 13, 1 << 23, 1 << 24] {
        assert!(parse(&with(0, d2, 0, 40), 0x3).is_none(), "{d2:#x}");
    }
}

#[test]
fn the_signal_is_the_best_of_the_chains_in_use() {
    let mut b = rx_buffer(&[0u8; 20], -70);
    b[29] = (2 * -40i32 + 220) as u8; // chain 1 stronger
    assert_eq!(parse(&b, 0x3).unwrap().rssi, -40);
    assert_eq!(parse(&b, 0x1).unwrap().rssi, -70, "one chain in use");
    b[28] = 230;
    b[29] = 240;
    assert_eq!(parse(&b, 0x3).unwrap().rssi, -128, "no chain below zero");
    assert_eq!(kind(&(6u32 << 27).to_le_bytes()), Kind::TxFree);
    assert_eq!(
        (kind(&(7u32 << 27 | 1 << 16).to_le_bytes()), kind(&[0, 0])),
        (Kind::Frame, Kind::Other)
    );
}
