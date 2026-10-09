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

//! A refused handshake is worded for the reader, and still refused.

use super::fetch_fixtures::{awaiting_flight, rfc_client, step};
use super::fetch_fixtures::{RFC_SERVER_FLIGHT, RFC_SERVER_HELLO};
use super::fetch_wire::FakeWire;
use crate::browser::fetch::retryable_error::retryable_error;
use crate::browser::fetch::security_error::security_error;
use crate::browser::fetch::tls::legacy::{selects_tls12, tls12_alert};
use crate::browser::fetch::tls_reason::{clock, reason, sentence};
use crate::browser::fetch::types::{Phase, TlsWhy};
use crate::browser::tls13::CertProblem;

/// A ServerHello record choosing `legacy`, with `exts` as its extensions
/// block (absent when `None`).
fn server_hello(legacy: u16, exts: Option<&[u8]>) -> Vec<u8> {
    let mut body = legacy.to_be_bytes().to_vec();
    body.extend_from_slice(&[0x11; 32]);
    body.push(0);
    body.extend_from_slice(&[0xc0, 0x2f, 0]);
    if let Some(e) = exts {
        body.extend_from_slice(&(e.len() as u16).to_be_bytes());
        body.extend_from_slice(e);
    }
    let len = (body.len() as u32).to_be_bytes();
    let msg = [&[2u8][..], &len[1..], &body].concat();
    [&[22u8, 3, 3][..], &(msg.len() as u16).to_be_bytes(), &msg].concat()
}

/* renegotiation_info, empty, as a 1.2 server commonly answers. */
const RENEGOTIATION: &[u8] = &[0xff, 0x01, 0, 1, 0];

#[test]
fn a_tls12_server_hello_is_recognised_however_it_says_so() {
    assert!(selects_tls12(&server_hello(0x0303, None)), "no extensions at all");
    assert!(selects_tls12(&server_hello(0x0303, Some(RENEGOTIATION))), "no supported_versions");
    let named = [0, 43, 0, 2, 3, 3];
    assert!(selects_tls12(&server_hello(0x0303, Some(&named))), "1.2 named in the extension");
    assert!(selects_tls12(&server_hello(0x0302, None)), "older still");
}

#[test]
fn a_tls13_server_hello_is_not_mistaken_for_one() {
    assert!(!selects_tls12(RFC_SERVER_HELLO));
    let named = [0, 43, 0, 2, 3, 4];
    assert!(!selects_tls12(&server_hello(0x0303, Some(&named))));
    let whole = server_hello(0x0303, None);
    for cut in 0..whole.len() {
        assert!(!selects_tls12(&whole[..cut]), "a hello cut at {cut} is not judged");
    }
    assert!(!selects_tls12(&[21, 3, 3, 0, 2, 2, 70]), "an alert is not a hello");
}

#[test]
fn only_the_alerts_a_tls12_server_sends_count() {
    assert!(tls12_alert(70) && tls12_alert(40));
    assert!(!tls12_alert(48) && !tls12_alert(0) && !tls12_alert(80));
}

#[test]
fn a_tls12_server_hello_ends_the_fetch_with_the_tls12_sentence() {
    let mut w = FakeWire::at(1_000);
    let mut f = awaiting_flight("https://old.example/", 7, 1_000, rfc_client());
    w.deliver(7, &server_hello(0x0303, Some(RENEGOTIATION)));
    step(&mut w, &mut f);
    assert_eq!((f.phase, f.error), (Phase::Error, Some("tls handshake refused")));
    assert_eq!(f.tls_why, Some(TlsWhy::Tls12Only));
    assert!(reason(&f).contains("only speaks TLS 1.2"));
}

#[test]
fn a_protocol_version_alert_ends_the_fetch_with_the_tls12_sentence() {
    let mut w = FakeWire::at(1_000);
    let mut f = awaiting_flight("https://old.example/", 7, 1_000, rfc_client());
    w.deliver(7, &[21, 3, 3, 0, 2, 2, 70]);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some("tls handshake refused"));
    assert_eq!((f.tls_why, f.tls_alert), (Some(TlsWhy::Tls12Only), Some(70)));
}

/* What a real TLS 1.2-only server (OpenSSL 3, `s_server -tls1_2`) answered
 * this client's hello with, recorded for the TLS proofs. */
const OPENSSL_TLS12_ALERT: &[u8] =
    include_bytes!("../../../tls_proofs/src/fixtures/openssl_tls12_alert.tls");

#[test]
fn a_real_tls12_only_server_is_worded_as_one() {
    let mut w = FakeWire::at(1_000);
    let mut f = awaiting_flight("https://old.example/", 7, 1_000, rfc_client());
    w.deliver(7, OPENSSL_TLS12_ALERT);
    step(&mut w, &mut f);
    assert_eq!(f.error, Some("tls handshake refused"));
    assert_eq!(f.tls_why, Some(TlsWhy::Tls12Only));
    assert_eq!(reason(&f), "This site only speaks TLS 1.2, which this browser does not support yet.");
}

#[test]
fn another_plain_alert_keeps_its_name() {
    let mut w = FakeWire::at(1_000);
    let mut f = awaiting_flight("https://x.example/", 7, 1_000, rfc_client());
    w.deliver(7, &[21, 3, 3, 0, 2, 2, 112]);
    step(&mut w, &mut f);
    assert_eq!(f.tls_why, None);
    assert_eq!(f.error, Some("tls handshake refused"));
    assert_eq!(
        reason(&f),
        "x.example refused the secure connection. It may need something this browser does not \
         offer. The server's alert was unrecognized_name."
    );
}

#[test]
fn a_refused_certificate_carries_its_reason_and_the_clock() {
    let mut w = FakeWire::at(1_000);
    let mut f = awaiting_flight("https://server/", 11, 1_000, rfc_client());
    w.deliver(11, &[RFC_SERVER_HELLO, RFC_SERVER_FLIGHT].concat());
    step(&mut w, &mut f);
    assert_eq!(f.error, Some("tls handshake refused"), "the refusal itself is unchanged");
    let Some(TlsWhy::Cert(problem, now)) = f.tls_why else {
        panic!("a certificate reason, got {:?}", f.tls_why)
    };
    assert!(problem.is_some(), "RFC 8448's self-made certificate has a reason to give");
    assert_eq!(now, 20260601000000);
    assert!(reason(&f).starts_with("certificate "));
}

#[test]
fn each_reason_reads_as_a_sentence_about_this_site() {
    let at = 20261003091500;
    let say = |p| sentence(TlsWhy::Cert(Some(p), at), "shop.example");
    assert!(say(CertProblem::UnknownIssuer).contains("authority this browser does not trust"));
    assert!(say(CertProblem::NameMismatch).ends_with("for a different name, not shop.example"));
    let expired = say(CertProblem::Expired);
    assert!(expired.contains("has expired") && expired.contains("2026-10-03 09:15 UTC"));
    let early = say(CertProblem::NotYetValid);
    assert!(early.contains("clock may be wrong") && early.contains("2026-10-03 09:15 UTC"));
    let tls12 = sentence(TlsWhy::Tls12Only, "old.example");
    assert_eq!(tls12, "This site only speaks TLS 1.2, which this browser does not support yet.");
}

#[test]
fn a_clock_that_could_not_be_read_is_said_so() {
    assert_eq!(clock(0), "the clock could not be read");
    assert_eq!(clock(20260101000000), "it reads 2026-01-01 00:00 UTC");
}

#[test]
fn no_handshake_sentence_is_ever_retried() {
    let whys = [
        TlsWhy::Tls12Only,
        TlsWhy::Cert(None, 0),
        TlsWhy::Cert(Some(CertProblem::Expired), 0),
        TlsWhy::Cert(Some(CertProblem::NotYetValid), 0),
        TlsWhy::Cert(Some(CertProblem::NameMismatch), 0),
        TlsWhy::Cert(Some(CertProblem::UnknownIssuer), 0),
        TlsWhy::Cert(Some(CertProblem::Unreadable), 0),
    ];
    for why in whys {
        let msg = sentence(why, "a.example");
        assert!(!retryable_error(&msg) || security_error(&msg), "{msg}");
        assert!(!msg.contains('\u{2014}'));
    }
}
