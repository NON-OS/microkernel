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

//! nonos_tls's public client stays TLS 1.3 only. net.anon carries its own
//! TLS 1.2 for relays that need it (capsule_net_anon/src/link/tls12), and
//! this is the proof that adding it gave the shared client no way down:
//!
//! - its hello offers TLS 1.3 alone, and only TLS 1.3 suites;
//! - a real TLS 1.2-only OpenSSL 3 server answers that hello with
//!   protocol_version (70), having read its supported_versions, recorded in
//!   fixtures/openssl_tls12_alert.tls by `record_openssl_tls12` against
//!   `openssl s_server -tls1_2`, and the client reports it as that alert. An
//!   OpenSSL 1.0.2 server, which does not read supported_versions, answers
//!   handshake_failure (40) for want of a shared suite; net.anon falls back
//!   to TLS 1.2 on either and on nothing else;
//! - a server that answers with a TLS 1.2 ServerHello anyway, echoing the
//!   session id and naming a TLS 1.3 suite, with or without a
//!   supported_versions of 0x0303, is refused.

use super::answering::Answering;
use super::hellos::{parse_hello, tls12_server_hello};
use crate::session::SessionError;
use crate::stream::connect_unauthenticated;

const OPENSSL_TLS12_ALERT: &[u8] = include_bytes!("../../fixtures/openssl_tls12_alert.tls");

#[test]
fn the_hello_offers_tls13_alone() {
    let mut io = Answering { answer: |_: &[u8]| Vec::new(), hello: None, queued: Vec::new() };
    let _ = connect_unauthenticated(&mut io, b"relay.example");
    let (_, suites, exts) = parse_hello(io.hello.as_ref().expect("a hello was written"));
    assert!(suites.iter().all(|s| (0x1301..=0x1305).contains(s)), "TLS 1.3 suites only: {suites:04x?}");
    let versions = &exts.iter().find(|(k, _)| *k == 43).expect("supported_versions").1;
    assert_eq!(versions, &[2, 3, 4], "TLS 1.3 and nothing else");
}

#[test]
fn a_real_tls12_server_refuses_it_and_the_client_says_so() {
    assert_eq!(OPENSSL_TLS12_ALERT, &[21, 3, 3, 0, 2, 2, 70], "OpenSSL's fatal protocol_version");
    let mut io = Answering { answer: |_: &[u8]| OPENSSL_TLS12_ALERT.to_vec(), hello: None, queued: Vec::new() };
    assert_eq!(connect_unauthenticated(&mut io, b"relay.example").err(), Some(SessionError::PeerAlert(70)));
}

#[test]
fn a_tls12_server_hello_is_refused_however_it_is_dressed() {
    let supported_12 = [0, 43, 0, 2, 3, 3];
    for (exts, label) in [(&[][..], "no extensions"), (&supported_12[..], "supported_versions 0x0303")] {
        for suite in [0x1301u16, 0x1303, 0xC030, 0xCCA8] {
            let mut io = Answering {
                answer: |hello: &[u8]| {
                    let (sid, _, _) = parse_hello(hello);
                    tls12_server_hello(&sid, suite, exts)
                },
                hello: None,
                queued: Vec::new(),
            };
            let got = connect_unauthenticated(&mut io, b"relay.example");
            assert!(got.is_err(), "{label}, suite {suite:04x}: a TLS 1.2 ServerHello was accepted");
        }
    }
}
