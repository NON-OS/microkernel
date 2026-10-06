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

//! A server that asks for a client certificate, as every Anyone relay does.
//!
//! The fork's tortls_openssl.c calls SSL_CTX_set_verify with SSL_VERIFY_PEER
//! on the server context too, so OpenSSL puts a CertificateRequest in every
//! relay's flight. The client answered it with a bare Finished, which the
//! relay reads as a missing Certificate and ends the link with an alert.
//! These proofs open the client's closing flight with the server's keys and
//! check it message by message.

use alloc::vec::Vec;

use super::fake_server::{handshake, Wire};
use crate::stream::connect_unauthenticated;

/// What OpenSSL 3 sends in the handshake: an empty context and one
/// signature_algorithms extension.
fn openssl_request() -> Vec<u8> {
    let mut algs = alloc::vec![0x00, 0x0d, 0x00, 0x08, 0x00, 0x06];
    algs.extend_from_slice(&[0x08, 0x04, 0x04, 0x03, 0x04, 0x01]);
    let mut body = alloc::vec![0u8];
    body.extend_from_slice(&(algs.len() as u16).to_be_bytes());
    body.extend_from_slice(&algs);
    handshake(13, &body)
}

/// The client's closing flight, decrypted under its handshake key, with the
/// transcript hash through the server Finished.
fn closing_flight(wire: &Wire) -> (Vec<u8>, crate::transcript::Transcript, crate::traffic_keys::TrafficKeys) {
    let (keys, transcript) = wire.handshake.clone().expect("the server answered");
    let record = &wire.from_client;
    assert_eq!(record[0], 23, "the closing flight is one protected record");
    let len = usize::from(u16::from_be_bytes([record[3], record[4]]));
    assert_eq!(record.len(), 5 + len, "and nothing else was written");
    let plain = crate::record_open::open(keys.suite, &keys.client_key, &keys.client_iv, 0, record)
        .expect("sealed under the client handshake key at sequence 0");
    let (inner, kind) = crate::inner_plain::split(&plain).expect("inner plaintext");
    assert_eq!(kind, 22, "handshake content");
    (inner.to_vec(), transcript, keys)
}

#[test]
fn a_certificate_request_is_answered_with_an_empty_certificate_then_finished() {
    let mut wire = Wire::new();
    wire.after_ee = openssl_request();
    assert!(connect_unauthenticated(&mut wire, b"relay.example").is_ok());
    let (inner, mut transcript, keys) = closing_flight(&wire);

    let certificate = [11u8, 0, 0, 4, 0, 0, 0, 0];
    assert_eq!(&inner[..8], &certificate, "empty context echoed, no certificates");
    transcript.push(&certificate);
    let mac = crate::finished_value::finished_value(&keys.client_secret, &transcript.digest()).expect("mac");
    assert_eq!(&inner[8..12], &[20, 0, 0, 32]);
    assert_eq!(&inner[12..], &mac, "the Finished covers the client's Certificate");
}

#[test]
fn a_request_context_is_echoed_byte_for_byte() {
    let mut wire = Wire::new();
    wire.after_ee = handshake(13, &[3, 0xA1, 0xB2, 0xC3, 0, 0]);
    assert!(connect_unauthenticated(&mut wire, b"relay.example").is_ok());
    let (inner, _, _) = closing_flight(&wire);
    assert_eq!(&inner[..11], &[11, 0, 0, 7, 3, 0xA1, 0xB2, 0xC3, 0, 0, 0]);
}

#[test]
fn without_a_request_the_flight_is_a_bare_finished() {
    let mut wire = Wire::new();
    assert!(connect_unauthenticated(&mut wire, b"relay.example").is_ok());
    let (inner, transcript, keys) = closing_flight(&wire);
    let mac = crate::finished_value::finished_value(&keys.client_secret, &transcript.digest()).expect("mac");
    assert_eq!(inner.len(), 36);
    assert_eq!(&inner[..4], &[20, 0, 0, 32]);
    assert_eq!(&inner[4..], &mac);
}

#[test]
fn a_second_request_ends_the_handshake() {
    let mut wire = Wire::new();
    wire.after_ee = [openssl_request(), openssl_request()].concat();
    assert!(connect_unauthenticated(&mut wire, b"relay.example").is_err());
}

#[test]
fn a_request_after_the_server_certificate_ends_the_handshake() {
    let mut wire = Wire::new();
    wire.after_cert = openssl_request();
    assert!(connect_unauthenticated(&mut wire, b"relay.example").is_err());
}

#[test]
fn a_request_whose_lengths_disagree_ends_the_handshake() {
    for body in [&[][..], &[4, 1, 2][..], &[0, 0, 5, 0, 0][..], &[0, 0, 0, 9][..]] {
        let mut wire = Wire::new();
        wire.after_ee = handshake(13, body);
        assert!(connect_unauthenticated(&mut wire, b"relay.example").is_err(), "{body:?}");
    }
}
