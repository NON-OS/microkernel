// NONOS Operating System (AGPL-3.0-or-later)

//! A frame's TXD and TXP against `mt76_connac2_mac_write_txwi` (the
//! 802.11 format, the MT7921 family's branches) and
//! `mt76_connac_write_hw_txp`; and the rates frames go at, by
//! `mt76_rates` and `mt76_connac2_mac_tx_rate_val`.

use crate::mac::rate::{basic, bitmaps, lowest, value};
use crate::mac::txd::{txwi, TxSpec};

fn w(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(b[4 * i..4 * i + 4].try_into().unwrap())
}

fn frame(fc: [u8; 2], a1: [u8; 6], len: usize) -> Vec<u8> {
    let mut f = vec![0u8; len];
    f[..2].copy_from_slice(&fc);
    f[4..10].copy_from_slice(&a1);
    f[22..24].copy_from_slice(&(0x123u16 << 4).to_le_bytes());
    f
}

const SPEC: TxSpec = TxSpec { wcid: 1, token: 0x2a, frame_dev: 0x1234_5000, rate: 0x4b };

#[test]
fn an_authentication_goes_to_altx0_at_a_fixed_rate_under_its_sequence() {
    let t = txwi(&frame([0xb0, 0], [2; 6], 30), &SPEC).unwrap();
    assert_eq!(w(&t, 0), (30 + 32) | 0x10 << 25, "bytes with the TXD, CT format, ALTX0");
    assert_eq!(w(&t, 1), 1 << 31 | 1 | 2 << 16 | 12 << 11, "long, wcid, 802.11, 24-byte header");
    assert_eq!(w(&t, 2), 0xb | 1 << 13 | 1 << 31, "subtype, type 0, HTC_VLD, FIX_RATE");
    assert_eq!(
        w(&t, 3),
        15 << 11 | 0x123 << 16 | 1 << 28 | 1 << 31,
        "retries, seq, no BA, SN_VALID"
    );
    assert_eq!((w(&t, 6), w(&t, 7)), (1 << 2 | 0x4b << 16, 0xb << 16));
    assert_eq!(u16::from_le_bytes([t[32], t[33]]), 0x2a | 1 << 15, "the token, valid");
    assert_eq!((w(&t, 10), u16::from_le_bytes([t[44], t[45]])), (0x1234_5000, 30 | 1 << 15));
}

#[test]
fn qos_data_to_a_group_rides_best_effort_with_its_tid_and_no_ack() {
    let mut f = frame([0x88, 0x01], [0xff; 6], 60);
    f[24] = 5;
    let t = txwi(&f, &SPEC).unwrap();
    assert_eq!(w(&t, 0) >> 25, 1, "MT_LMAC_AC01, best effort");
    assert_eq!((w(&t, 1) >> 11 & 0x1f, w(&t, 1) >> 20 & 7), (13, 5), "26-byte header, TID 5");
    assert_eq!((w(&t, 2) >> 10 & 1, w(&t, 3) & 1), (1, 1), "multicast, no ack");
}

#[test]
fn a_fragment_or_a_frame_the_txp_cannot_say_is_refused() {
    let mut more = frame([0x08, 0x04], [2; 6], 40);
    assert!(txwi(&more, &SPEC).is_none(), "More Fragments");
    more[1] = 0;
    more[22] |= 1;
    assert!(txwi(&more, &SPEC).is_none(), "fragment number 1");
    assert!(txwi(&frame([0x08, 0], [2; 6], 4096), &SPEC).is_none(), "twelve bits");
    assert!(txwi(&frame([0x08, 0], [2; 6], 4095), &SPEC).is_some());
    assert!(txwi(&[0x08, 0, 0], &SPEC).is_none(), "shorter than a header");
}

#[test]
fn rates_are_the_chips_values_and_basic_ones_are_the_bands_own() {
    assert_eq!(
        (value(2, false), value(22, false), value(12, false), value(108, true)),
        (0, 3, 0x4b, 0x4c)
    );
    assert_eq!((value(2, true), lowest(true), lowest(false)), (0x4b, 12, 2), "no CCK on 5 GHz");
    let ap = [0x82, 0x84, 0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24];
    assert_eq!(bitmaps(&ap, false), (0xff, 0xf));
    assert_eq!(bitmaps(&[0x8c, 0x98, 0x30], true), (0x15, 0x05));
    assert_eq!((basic(&ap, false), basic(&[0x0c, 0x98], true)), (Some((2, 22)), Some((24, 24))));
    assert_eq!(basic(&ap, true), None, "no CCK rate is a 5 GHz basic rate");
}
