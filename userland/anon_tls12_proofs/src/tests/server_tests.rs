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


//! The server's messages, each refused for exactly what the client did not
//! offer or cannot read.

use std::vec::Vec;

use super::cases::{get, handshake};
use crate::tls12::server::{certificate, certificate_request, key_exchange, server_hello};
use crate::tls12::Tls12Error;

fn hello(suite: u16, compression: u8, exts: &[(u16, &[u8])]) -> Vec<u8> {
    let mut b = std::vec![3, 3];
    b.extend_from_slice(&[0x42; 32]);
    b.push(0);
    b.extend_from_slice(&suite.to_be_bytes());
    b.push(compression);
    let mut e = Vec::new();
    for (kind, data) in exts {
        e.extend_from_slice(&kind.to_be_bytes());
        e.extend_from_slice(&(data.len() as u16).to_be_bytes());
        e.extend_from_slice(data);
    }
    b.extend_from_slice(&(e.len() as u16).to_be_bytes());
    b.extend_from_slice(&e);
    b
}

#[test]
fn a_good_server_hello_reads() {
    let h = server_hello(&hello(0xCCA8, 0, &[(23, &[]), (0xff01, &[0]), (11, &[1, 0])])).unwrap();
    assert_eq!(h.suite, 0xCCA8);
    assert!(h.ems);
    assert!(!server_hello(&hello(0xC030, 0, &[])).unwrap().ems);
}

#[test]
fn what_was_not_offered_is_refused() {
    for suite in [0xC02F, 0x009C, 0x1301, 0xC014, 0x0000] {
        assert_eq!(server_hello(&hello(suite, 0, &[])).err(), Some(Tls12Error::Unoffered), "suite {suite:04x}");
    }
    assert_eq!(server_hello(&hello(0xCCA8, 1, &[])).err(), Some(Tls12Error::Unoffered), "compression");
    for ext in [35u16, 43, 16, 22, 51, 5] {
        assert_eq!(server_hello(&hello(0xCCA8, 0, &[(ext, &[])])).err(), Some(Tls12Error::Unoffered), "extension {ext}");
    }
    let mut v = hello(0xCCA8, 0, &[]);
    v[1] = 2;
    assert_eq!(server_hello(&v).err(), Some(Tls12Error::Unoffered), "TLS 1.1");
    v[1] = 4;
    assert_eq!(server_hello(&v).err(), Some(Tls12Error::Unoffered), "a TLS 1.3 legacy version");
}

#[test]
fn malformed_answers_to_what_was_offered_are_refused() {
    for exts in [
        std::vec![(23u16, &[0u8][..])],
        std::vec![(0xff01, &[1, 7][..])],
        std::vec![(0xff01, &[][..])],
        std::vec![(11, &[1, 1][..])],
        std::vec![(11, &[2, 0][..])],
        std::vec![(0, &[0][..])],
        std::vec![(23, &[][..]), (23, &[][..])],
    ] {
        assert_eq!(server_hello(&hello(0xCCA8, 0, &exts)).err(), Some(Tls12Error::Malformed), "{exts:?}");
    }
}

#[test]
fn the_downgrade_mark_is_refused_and_its_tls11_cousin_is_not_special() {
    let mut v = hello(0xCCA8, 0, &[]);
    v[2 + 24..2 + 32].copy_from_slice(b"DOWNGRD\x01");
    assert_eq!(server_hello(&v).err(), Some(Tls12Error::Downgrade));
    v[2 + 31] = 0;
    assert!(server_hello(&v).is_ok(), "DOWNGRD\\0 marks TLS 1.1 and below, which this hello never got to");
}

#[test]
fn every_truncation_of_every_server_message_is_refused_without_panic() {
    let h = hello(0xCCA8, 0, &[(23, &[])]);
    for n in 0..h.len() {
        // Cut right after compression it is a whole hello with no extensions,
        // which TLS allows; it then has no extended master secret.
        if n == 38 {
            assert!(!server_hello(&h[..n]).unwrap().ems);
            continue;
        }
        assert!(server_hello(&h[..n]).is_err(), "server hello cut to {n}");
    }
    let mut long = h.clone();
    long.push(0);
    assert!(server_hello(&long).is_err(), "trailing byte");
    for (kind, body) in handshake(get("chacha_certreq").server) {
        for n in 0..body.len() {
            let r = match kind {
                11 => certificate(&body[..n]).map(|_| ()),
                12 => key_exchange(&body[..n]).map(|_| ()),
                13 => certificate_request(&body[..n]),
                _ => continue,
            };
            assert!(r.is_err(), "message {kind} cut to {n}");
        }
    }
}

#[test]
fn the_key_exchange_must_be_p256_named_and_uncompressed_with_an_offered_scheme() {
    let (_, ske) = handshake(get("chacha_certreq").server).into_iter().find(|(k, _)| *k == 12).unwrap();
    assert!(key_exchange(&ske).is_ok());
    let mut b = ske.clone();
    b[0] = 1;
    assert_eq!(key_exchange(&b).err(), Some(Tls12Error::Unoffered), "explicit curve");
    let mut b = ske.clone();
    b[2] = 24;
    assert_eq!(key_exchange(&b).err(), Some(Tls12Error::Unoffered), "P-384");
    let mut b = ske.clone();
    b[4] = 2;
    assert_eq!(key_exchange(&b).err(), Some(Tls12Error::Malformed), "compressed point");
    for scheme in [[2u8, 1], [6, 1], [8, 6], [4, 3], [8, 7]] {
        let mut b = ske.clone();
        b[69..71].copy_from_slice(&scheme);
        assert_eq!(key_exchange(&b).err(), Some(Tls12Error::Unoffered), "scheme {scheme:?}");
    }
}

#[test]
fn an_empty_certificate_list_is_refused() {
    assert_eq!(certificate(&[0, 0, 0]).err(), Some(Tls12Error::Malformed));
    assert_eq!(certificate(&[0, 0, 3, 0, 0, 0]).err(), Some(Tls12Error::Malformed));
}
