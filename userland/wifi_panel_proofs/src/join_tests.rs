// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Proofs for the client's join and link messages: the join body carries the
//! saved flags in the octet the RTL8821CE's `parse_connect` reads, replies are
//! read field by field only as far as the driver sent them (an older driver's
//! shorter reply reads as zero there), every truncation of a reply parses or
//! is refused without a panic, and the scan flags name WPA2 and WPA3.

use crate::wifi::join_wire::{
    encode_join, parse_connect_reply, parse_link, JoinFlags, AKM_SAE, JOIN_BODY_MAX,
};
use crate::wifi::parse_scan;

#[test]
fn the_join_body_ends_with_the_flags_octet() {
    let mut out = [0u8; JOIN_BODY_MAX];
    let n = encode_join(b"Home", b"pw12345678", JoinFlags { wpa3_only: true, hidden: false }, &mut out);
    assert_eq!(&out[..n], b"\x04Home\x0apw12345678\x01");
    let n = encode_join(b"H", b"", JoinFlags { wpa3_only: true, hidden: true }, &mut out);
    assert_eq!(&out[..n], &[1, b'H', 0, 0x03]);
    let n = encode_join(b"H", b"", JoinFlags::default(), &mut out);
    assert_eq!(out[n - 1], 0, "no flags: an older driver's reading");
}

#[test]
fn the_longest_join_fits_and_overlong_fields_are_cut() {
    let mut out = [0u8; JOIN_BODY_MAX];
    let n = encode_join(&[b's'; 40], &[b'p'; 80], JoinFlags::default(), &mut out);
    assert_eq!(n, JOIN_BODY_MAX);
    assert_eq!((out[0], out[33]), (32, 64));
}

// A connect reply body from a driver that runs WPA3.
fn full_reply() -> Vec<u8> {
    let mut b = Vec::new();
    b.extend_from_slice(&(-10i32).to_le_bytes());
    for v in 1u32..=7 {
        b.extend_from_slice(&v.to_le_bytes());
    }
    b.push(4); // state
    b.push(AKM_SAE);
    b.extend_from_slice(&15u16.to_le_bytes());
    b
}

#[test]
fn a_connect_reply_carries_the_akm_and_the_access_points_code() {
    let r = parse_connect_reply(&full_reply()).expect("parses");
    assert_eq!((r.code, r.sent, r.to_us, r.state), (-10, 1, 7, 4));
    assert_eq!((r.akm, r.ap_code), (AKM_SAE, 15));
}

#[test]
fn an_older_drivers_reply_reads_as_zero_past_its_end() {
    let b = full_reply();
    let r = parse_connect_reply(&b[..33]).expect("the counters and state");
    assert_eq!((r.state, r.akm, r.ap_code), (4, 0, 0));
    let r = parse_connect_reply(&b[..4]).expect("a bare status");
    assert_eq!((r.code, r.sent), (-10, 0));
    assert!(parse_connect_reply(&b[..3]).is_none(), "no status at all");
    for n in 0..=b.len() {
        let _ = parse_connect_reply(&b[..n]);
    }
}

// A link reply body for `ssid`, with `akm` when given.
fn link_body(ssid: &[u8], akm: Option<u8>) -> Vec<u8> {
    let mut b = vec![1, 2, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, ssid.len() as u8];
    b.extend_from_slice(ssid);
    b.extend(akm);
    b
}

#[test]
fn a_link_reply_says_whether_wpa3_ran() {
    let l = parse_link(&link_body(b"Home", Some(AKM_SAE))).expect("parses");
    assert!(l.associated);
    assert_eq!(l.ssid(), b"Home");
    assert!(l.joined_with_sae(b"Home"));
    assert!(!l.joined_with_sae(b"Other"), "another network's name");
    let wpa2 = parse_link(&link_body(b"Home", Some(2))).unwrap();
    assert!(!wpa2.joined_with_sae(b"Home"));
    let old = parse_link(&link_body(b"Home", None)).unwrap();
    assert_eq!(old.akm, 0, "an older driver does not say");
}

#[test]
fn a_cut_or_lying_link_reply_is_read_safely() {
    let b = link_body(b"Home", Some(AKM_SAE));
    assert!(parse_link(&b[..7]).is_none(), "shorter than the fixed fields");
    for n in 8..=b.len() {
        let l = parse_link(&b[..n]).expect("the fixed fields are there");
        assert!(l.ssid().len() <= 4);
        if n < b.len() {
            assert_eq!(l.akm, 0, "no AKM is read from a cut reply");
        }
    }
    let mut lie = link_body(b"Home", None);
    lie[7] = 200; // an SSID length past the reply and past 32
    let l = parse_link(&lie).unwrap();
    assert_eq!((l.ssid(), l.akm), (&b"Home"[..], 0));
}

#[test]
fn the_scan_flags_name_wpa2_and_wpa3() {
    let buf = [3, 50, 0x03, 1, b'a', 60, 0x05, 1, b'b', 70, 0x07, 1, b'c'];
    let mut got = Vec::new();
    assert_eq!(parse_scan(&buf, |n| got.push(n)), 3);
    assert!(got[0].secured && got[0].wpa2 && !got[0].wpa3, "WPA2 only");
    assert!(got[1].wpa3 && !got[1].wpa2, "WPA3 only");
    assert!(got[2].wpa2 && got[2].wpa3, "a transition network");
    let old = [1, 50, 0x01, 1, b'a'];
    parse_scan(&old, |n| assert!(n.secured && !n.wpa2 && !n.wpa3, "a driver that does not say"));
}
