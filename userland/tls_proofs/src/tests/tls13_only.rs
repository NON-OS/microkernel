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

use std::vec::Vec;

use crate::session::{Io, SessionError};
use crate::stream::connect_unauthenticated;

const OPENSSL_TLS12_ALERT: &[u8] = include_bytes!("../fixtures/openssl_tls12_alert.tls");

/// A server that answers the client's hello with `answer(hello)`.
struct Answering<F: FnMut(&[u8]) -> Vec<u8>> {
    answer: F,
    hello: Option<Vec<u8>>,
    queued: Vec<u8>,
}

impl<F: FnMut(&[u8]) -> Vec<u8>> Io for Answering<F> {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        if self.hello.is_none() {
            self.hello = Some(data.to_vec());
            self.queued = (self.answer)(data);
        }
        Ok(())
    }
    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        if self.queued.is_empty() {
            return Err(SessionError::Io);
        }
        let n = into.len().min(self.queued.len());
        into[..n].copy_from_slice(&self.queued[..n]);
        self.queued.drain(..n);
        Ok(n)
    }
}

/// The hello's legacy session id, cipher suites and extensions.
type Hello = (Vec<u8>, Vec<u16>, Vec<(u16, Vec<u8>)>);

fn parse_hello(record: &[u8]) -> Hello {
    let body = &record[5 + 4..];
    let sid_len = usize::from(body[34]);
    let sid = body[35..35 + sid_len].to_vec();
    let mut at = 35 + sid_len;
    let n = usize::from(u16::from_be_bytes([body[at], body[at + 1]]));
    let suites = body[at + 2..at + 2 + n].chunks(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();
    at += 2 + n;
    at += 1 + usize::from(body[at]);
    let end = at + 2 + usize::from(u16::from_be_bytes([body[at], body[at + 1]]));
    at += 2;
    let mut exts = Vec::new();
    while at < end {
        let kind = u16::from_be_bytes([body[at], body[at + 1]]);
        let len = usize::from(u16::from_be_bytes([body[at + 2], body[at + 3]]));
        exts.push((kind, body[at + 4..at + 4 + len].to_vec()));
        at += 4 + len;
    }
    (sid, suites, exts)
}

/// A TLS 1.2 ServerHello echoing `sid`, naming `suite`, with `exts`.
fn tls12_server_hello(sid: &[u8], suite: u16, exts: &[u8]) -> Vec<u8> {
    let mut body = std::vec![3, 3];
    body.extend_from_slice(&[0x5a; 32]);
    body.push(sid.len() as u8);
    body.extend_from_slice(sid);
    body.extend_from_slice(&suite.to_be_bytes());
    body.push(0);
    if !exts.is_empty() {
        body.extend_from_slice(&(exts.len() as u16).to_be_bytes());
        body.extend_from_slice(exts);
    }
    let mut msg = std::vec![2, 0, (body.len() >> 8) as u8, body.len() as u8];
    msg.extend_from_slice(&body);
    let mut record = std::vec![22, 3, 3, (msg.len() >> 8) as u8, msg.len() as u8];
    record.extend_from_slice(&msg);
    record
}

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

/// Records fixtures/openssl_tls12_alert.tls, on demand only:
///   openssl s_server -accept 44400 -tls1_2 -cert c.pem -key k.pem &
///   NONOS_TLS12_SERVER=127.0.0.1:44400 cargo test --release record_openssl_tls12 -- --ignored
#[test]
#[ignore = "needs openssl s_server -tls1_2 listening"]
fn record_openssl_tls12() {
    use std::io::{Read, Write};
    let addr = std::env::var("NONOS_TLS12_SERVER").expect("ip:port");
    let mut tcp = std::net::TcpStream::connect(addr).expect("s_server");
    tcp.set_read_timeout(Some(std::time::Duration::from_secs(3))).unwrap();
    let mut io = Answering { answer: |_: &[u8]| Vec::new(), hello: None, queued: Vec::new() };
    let _ = connect_unauthenticated(&mut io, b"relay.example");
    tcp.write_all(io.hello.as_ref().unwrap()).unwrap();
    let mut answer = std::vec![0u8; 4096];
    let n = tcp.read(&mut answer).unwrap();
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/fixtures/openssl_tls12_alert.tls");
    std::fs::write(path, &answer[..n]).unwrap();
}
