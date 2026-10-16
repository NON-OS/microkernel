// NONOS Operating System (AGPL-3.0-or-later)
//! Which requests go where, and the connect body read with every length
//! checked.


use crate::control::{answer, route, Route, View, REPLY_MAX};
use crate::join_wire::*;

fn control(op: u16) -> Vec<u8> {
    let mut v = 0x5749_4649u32.to_le_bytes().to_vec();
    v.extend_from_slice(&op.to_le_bytes());
    v.extend_from_slice(&7u32.to_le_bytes());
    v
}

#[test]
fn requests_go_to_the_link_the_radio_or_as_before() {
    let mut nnet = 0x4E4E_4554u32.to_le_bytes().to_vec();
    nnet.extend_from_slice(&[0; 16]);
    for can in [false, true] {
        assert_eq!(route(&nnet, can), Route::Link, "net_core's link protocol");
        assert_eq!(route(&control(3), can), Route::Control, "scan");
        assert_eq!(route(&control(4), can), Route::Control, "status");
        assert_eq!(route(&control(4)[..9], can), Route::Driver, "shorter than the header");
        assert_eq!(route(&0x4E49_5746u32.to_le_bytes(), can), Route::Driver, "this driver's own");
        assert_eq!(route(&[1, 2], can), Route::Driver);
    }
    for op in [1u16, 2, 5] {
        assert_eq!(route(&control(op), true), Route::Join(op));
        assert_eq!(route(&control(op), false), Route::Control, "a radio that cannot join answers as before");
    }
    // Answered as before: -38, and nothing of the body is kept.
    let view = View { stage: 0, step: 0, detail: 0, hw_rev: 0, rf_id: 0, heard: None };
    let mut out = vec![0u8; REPLY_MAX];
    let n = answer(&control(1), &view, &mut out).unwrap();
    assert_eq!((n, i32::from_le_bytes(out[10..14].try_into().unwrap())), (14, -38));
}

#[test]
fn a_connect_body_is_read_with_every_length_checked() {
    let mut b = vec![4];
    b.extend_from_slice(b"home");
    b.push(8);
    b.extend_from_slice(b"passw0rd");
    let r = parse_connect(&b).unwrap();
    assert_eq!((r.ssid, r.pass, r.wpa3_only, r.hidden), (&b"home"[..], &b"passw0rd"[..], false, false));
    b.push(0b11);
    let r = parse_connect(&b).unwrap();
    assert!(r.wpa3_only && r.hidden);
    assert!(parse_connect(&b[..5]).is_none(), "no passphrase length");
    assert!(parse_connect(&b[..12]).is_none(), "the passphrase runs past the body");
    assert!(parse_connect(&[0, 0]).is_none(), "an empty SSID");
    let mut long = vec![33];
    long.extend_from_slice(&[b'a'; 33]);
    long.push(0);
    assert!(parse_connect(&long).is_none(), "an SSID longer than 32");
    assert!(parse_connect(&[]).is_none());
}
