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

//! Proofs for the connect request the serve stage parses and the status codes
//! a join reports back. The request body comes over IPC from any client, so
//! every length in it is checked before use; the flags octet is optional so a
//! client that predates it still connects. Every way a join can end maps to a
//! code the client can name, with the WPA3 outcomes (a downgrade refused, a
//! wrong SAE password, a message 3 that did not match the beacon) kept apart
//! from a plain timeout. Before a join the hunt transmits nothing for a
//! network that broadcasts its name; only a hidden one is probed, by name.

use crate::connect_probe::{hunt_probe, PROBE_RATES};
use crate::connect_request::{parse_connect, FLAG_HIDDEN, FLAG_WPA3_ONLY};
use crate::connect_result::*;
use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::rsn::SelectError;
use nonos_wifi_core::sae::SaeFailure;
use nonos_wifi_core::wpa::supplicant::Failure;

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

#[test]
fn each_way_a_join_ends_has_its_own_code() {
    let cases = [
        (MlmeFailure::Select(SelectError::Downgrade), (CODE_DOWNGRADE, 0)),
        (MlmeFailure::Select(SelectError::UnsupportedAkm), (CODE_UNSUPPORTED, 0)),
        (MlmeFailure::Select(SelectError::UnsupportedCipher), (CODE_UNSUPPORTED, 0)),
        (MlmeFailure::OpenNetwork, (CODE_UNSUPPORTED, 0)),
        (MlmeFailure::MalformedRsne, (CODE_UNSUPPORTED, 0)),
        (MlmeFailure::NeedsHt, (CODE_UNSUPPORTED, 0)),
        (MlmeFailure::BadPassphrase, (CODE_BAD_PASSPHRASE, 0)),
        (MlmeFailure::NoEntropy, (CODE_NO_ENTROPY, 0)),
        (MlmeFailure::Sae(SaeFailure::BadConfirm), (CODE_WRONG_PASSWORD, 0)),
        (MlmeFailure::Sae(SaeFailure::Rejected(77)), (CODE_REFUSED, 77)),
        (MlmeFailure::Sae(SaeFailure::GroupNotSupported), (CODE_REFUSED, 0)),
        (MlmeFailure::AuthRejected(1), (CODE_REFUSED, 1)),
        (MlmeFailure::AssocRejected(17), (CODE_REFUSED, 17)),
        (MlmeFailure::Left(15), (CODE_TIMED_OUT, 15)),
        (MlmeFailure::Handshake(Failure::IeMismatch), (CODE_IE_MISMATCH, 0)),
        (MlmeFailure::Handshake(Failure::BadKeyData), (CODE_TIMED_OUT, 0)),
    ];
    for (f, want) in cases {
        assert_eq!(failure_code(f), want);
    }
}

#[test]
fn the_codes_keep_the_meanings_the_client_already_shows() {
    assert_eq!(
        [CODE_BAD_REQUEST, CODE_NOT_FOUND, CODE_REFUSED, CODE_TIMED_OUT],
        [-1, -2, -5, -6],
        "the existing codes are unchanged"
    );
    let new = [
        CODE_DOWNGRADE,
        CODE_UNSUPPORTED,
        CODE_BAD_PASSPHRASE,
        CODE_WRONG_PASSWORD,
        CODE_IE_MISMATCH,
        CODE_NO_ENTROPY,
    ];
    assert_eq!(new, [-7, -8, -9, -10, -11, -12], "the new ones follow them");
    let r = ConnectResult::code(CODE_NOT_FOUND);
    assert_eq!((r.code, r.akm, r.ap_code, r.sent), (CODE_NOT_FOUND, 0, 0, 0));
}

#[test]
fn only_a_hidden_network_is_probed_and_only_by_its_own_name() {
    // A per-boot address: locally administered, unicast.
    let mac = [0x06, 0x11, 0x22, 0x33, 0x44, 0x55];
    let mut out = [0u8; 64];
    for step in 0..13 {
        assert_eq!(hunt_probe(&mut out, false, mac, b"Home", step), None, "a named network is heard, not asked for");
    }
    let n = hunt_probe(&mut out, true, mac, b"Home", 3).expect("a hidden network gets a directed probe");
    let f = &out[..n];
    assert_eq!(f[0], 0x40, "a probe request");
    assert_eq!(&f[4..10], &[0xFF; 6], "to broadcast");
    assert_eq!(&f[10..16], &mac, "from the per-boot address");
    assert_eq!(&f[24..30], &[0, 4, b'H', b'o', b'm', b'e'], "naming the hidden network and nothing else");
    assert_eq!((f[30], f[31]), (1, PROBE_RATES.len() as u8), "then the rates");
    assert_eq!(n, 32 + PROBE_RATES.len());
    // A buffer that cannot hold the frame gets nothing, not a cut frame.
    assert_eq!(hunt_probe(&mut [0u8; 30], true, mac, b"Home", 0), None);
}
