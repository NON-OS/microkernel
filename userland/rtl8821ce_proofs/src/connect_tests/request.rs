// NONOS Operating System (AGPL-3.0-or-later)

//! The connect request's body: flags, and every malformed one refused.

use crate::connect_request::{parse_connect, FLAG_HIDDEN, FLAG_WPA3_ONLY};

fn body(ssid: &[u8], pass: &[u8], flags: Option<u8>) -> Vec<u8> {
    let mut b = vec![ssid.len() as u8];
    b.extend_from_slice(ssid);
    b.push(pass.len() as u8);
    b.extend_from_slice(pass);
    b.extend(flags);
    b
}

#[test]
fn a_request_without_flags_has_no_constraints() {
    let b = body(b"Home", b"password123", None);
    let r = parse_connect(&b).expect("parses");
    assert_eq!(r.ssid, b"Home");
    assert_eq!(r.pass, b"password123");
    assert!(!r.wpa3_only && !r.hidden, "an older client's request joins as before");
}

#[test]
fn the_flags_octet_carries_wpa3_only_and_hidden() {
    let b = body(b"Home", b"pw", Some(FLAG_WPA3_ONLY));
    let r = parse_connect(&b).unwrap();
    assert!(r.wpa3_only && !r.hidden);
    let b = body(b"Home", b"pw", Some(FLAG_WPA3_ONLY | FLAG_HIDDEN));
    let r = parse_connect(&b).unwrap();
    assert!(r.wpa3_only && r.hidden);
    let b = body(b"Home", b"", Some(FLAG_HIDDEN));
    let r = parse_connect(&b).unwrap();
    assert!(r.pass.is_empty() && r.hidden && !r.wpa3_only);
}

#[test]
fn malformed_requests_are_refused() {
    assert!(parse_connect(&[]).is_none(), "empty");
    assert!(parse_connect(&body(b"", b"pw", None)).is_none(), "no SSID");
    assert!(parse_connect(&body(&[b'x'; 33], b"pw", None)).is_none(), "an SSID over 32 octets");
    // Lengths running past the body.
    assert!(parse_connect(&[5, b'a', b'b']).is_none(), "the SSID runs past the end");
    assert!(parse_connect(&[2, b'a', b'b']).is_none(), "no passphrase length");
    assert!(parse_connect(&[2, b'a', b'b', 9, b'p']).is_none(), "the passphrase runs past");
    // Bytes past the flags are ignored, not an error.
    let mut b = body(b"ab", b"pw", Some(0));
    b.extend_from_slice(&[0xFF; 4]);
    assert!(parse_connect(&b).is_some());
}

#[test]
fn every_request_prefix_parses_or_is_refused_without_a_panic() {
    let b = body(&[b's'; 32], &[b'p'; 63], Some(0xFF));
    for n in 0..=b.len() {
        let _ = parse_connect(&b[..n]);
    }
}
