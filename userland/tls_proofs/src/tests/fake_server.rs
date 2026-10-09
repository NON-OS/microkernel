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

//! A TLS 1.3 server, the smallest that a real client completes against.
//!
//! It answers whatever ClientHello it is sent with an X25519 ServerHello, an
//! empty EncryptedExtensions, a certificate for a P-256 key it holds, a
//! CertificateVerify signed with that key and a Finished, all built from the
//! client's own key schedule, so a session opened against it is a real one.
//! What the proofs then send down that session is up to them.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use p256::ecdsa::signature::Signer;
use p256::ecdsa::{Signature, SigningKey};

use super::der_build::{seq, tlv};
use crate::session::{Io, SessionError};
use crate::traffic_keys::TrafficKeys;
use crate::transcript::Transcript;

const SUITE: u16 = 0x1301;

/// The wire between the client under test and this server.
pub(crate) struct Wire {
    to_client: VecDeque<u8>,
    /// The server's application keys, once the handshake is built.
    pub(crate) app: Option<TrafficKeys>,
    /// The next application record sequence number the server uses.
    pub(crate) seq: u64,
    /// Raw handshake messages sent right after EncryptedExtensions, such as
    /// a CertificateRequest.
    pub(crate) after_ee: Vec<u8>,
    /// Raw handshake messages sent right after the server's Certificate.
    pub(crate) after_cert: Vec<u8>,
    /// Everything the client wrote after its ClientHello.
    pub(crate) from_client: Vec<u8>,
    /// The server's view of the handshake keys and of the transcript through
    /// its Finished, so a proof can open and check the client's reply.
    pub(crate) handshake: Option<(TrafficKeys, Transcript)>,
}

impl Wire {
    pub(crate) fn new() -> Self {
        Wire {
            to_client: VecDeque::new(),
            app: None,
            seq: 0,
            after_ee: Vec::new(),
            after_cert: Vec::new(),
            from_client: Vec::new(),
            handshake: None,
        }
    }

    /// Put raw bytes on the wire towards the client.
    pub(crate) fn send_raw(&mut self, bytes: &[u8]) {
        self.to_client.extend(bytes.iter().copied());
    }

    /// Seal `body` as one application record of inner type `kind` and send it.
    pub(crate) fn send_record(&mut self, kind: u8, body: &[u8]) {
        let app = self.app.expect("the handshake is done");
        let record = crate::record_seal::seal(app.suite, &app.server_key, &app.server_iv, self.seq, kind, body)
            .expect("seal");
        self.seq += 1;
        self.send_raw(&record);
    }
}

impl Io for Wire {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        if self.app.is_none() {
            let (flight, app, handshake) = answer(data, &self.after_ee, &self.after_cert);
            self.app = Some(app);
            self.handshake = Some(handshake);
            self.send_raw(&flight);
        } else {
            self.from_client.extend_from_slice(data);
        }
        Ok(())
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        let n = into.len().min(self.to_client.len());
        for (slot, byte) in into.iter_mut().zip(self.to_client.drain(..n)) {
            *slot = byte;
        }
        Ok(n)
    }
}

fn u24(n: usize) -> [u8; 3] {
    [(n >> 16) as u8, (n >> 8) as u8, n as u8]
}

pub(crate) fn handshake(kind: u8, body: &[u8]) -> Vec<u8> {
    let mut m = alloc::vec![kind];
    m.extend_from_slice(&u24(body.len()));
    m.extend_from_slice(body);
    m
}

/// The client's X25519 share and session id, out of its ClientHello record.
fn client_share(record: &[u8]) -> (Vec<u8>, [u8; 32], [u8; 32]) {
    let ch = record[5..].to_vec();
    let body = &ch[4..];
    let mut sid = [0u8; 32];
    sid.copy_from_slice(&body[35..67]);
    // version, random, then the session id: its length at 34, itself at 35.
    let mut at = 35 + 32;
    let suites = usize::from(u16::from_be_bytes([body[at], body[at + 1]]));
    at += 2 + suites;
    at += 1 + usize::from(body[at]);
    let end = at + 2 + usize::from(u16::from_be_bytes([body[at], body[at + 1]]));
    at += 2;
    while at < end {
        let kind = u16::from_be_bytes([body[at], body[at + 1]]);
        let len = usize::from(u16::from_be_bytes([body[at + 2], body[at + 3]]));
        if kind == 0x0033 {
            let shares = &body[at + 6..at + 4 + len];
            let mut s = 0;
            while s < shares.len() {
                let group = u16::from_be_bytes([shares[s], shares[s + 1]]);
                let klen = usize::from(u16::from_be_bytes([shares[s + 2], shares[s + 3]]));
                if group == 0x001d {
                    let mut key = [0u8; 32];
                    key.copy_from_slice(&shares[s + 4..s + 4 + klen]);
                    return (ch, key, sid);
                }
                s += 4 + klen;
            }
        }
        at += 4 + len;
    }
    panic!("the client offered an X25519 share")
}

/// A certificate for `point`, enough for the unauthenticated path, which
/// checks the CertificateVerify against it and no chain.
fn certificate(point: &[u8]) -> Vec<u8> {
    let ec = [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x02, 0x01];
    let p256 = [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x03, 0x01, 0x07];
    let ecdsa_sha256 = [0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x04, 0x03, 0x02];
    let mut bits = alloc::vec![0u8];
    bits.extend_from_slice(point);
    let spki = seq(&[&seq(&[&tlv(0x06, &ec), &tlv(0x06, &p256)]), &tlv(0x03, &bits)]);
    let alg = seq(&[&tlv(0x06, &ecdsa_sha256)]);
    let name = seq(&[&tlv(0x31, &seq(&[&tlv(0x06, &[0x55, 0x04, 0x03]), &tlv(0x0c, b"relay")]))]);
    let validity = seq(&[&tlv(0x17, b"260101000000Z"), &tlv(0x17, b"270101000000Z")]);
    let tbs = seq(&[&tlv(0xa0, &tlv(0x02, &[2])), &tlv(0x02, &[1]), &alg, &name, &validity, &name, &spki]);
    seq(&[&tbs, &alg, &tlv(0x03, &[0, 0x30, 0x00])])
}

/// The server's whole first flight for this ClientHello record, and the
/// application keys it leads to.
fn answer(client_record: &[u8], after_ee: &[u8], after_cert: &[u8]) -> (Vec<u8>, TrafficKeys, (TrafficKeys, Transcript)) {
    let (ch, client_pub, sid) = client_share(client_record);
    let private = [0x42u8; 32];
    let mut public = [0u8; 32];
    assert_eq!(nonos_libc::crypto_x25519_public(private.as_ptr(), public.as_mut_ptr()), 32);
    let mut shared = [0u8; 32];
    assert_eq!(nonos_libc::crypto_x25519_shared(private.as_ptr(), client_pub.as_ptr(), shared.as_mut_ptr()), 32);

    let mut sh = alloc::vec![0x03, 0x03];
    sh.extend_from_slice(&[0x5A; 32]);
    sh.push(32);
    sh.extend_from_slice(&sid);
    sh.extend_from_slice(&SUITE.to_be_bytes());
    sh.push(0);
    let mut ext = alloc::vec![0x00, 0x2b, 0x00, 0x02, 0x03, 0x04, 0x00, 0x33, 0x00, 0x24, 0x00, 0x1d, 0x00, 0x20];
    ext.extend_from_slice(&public);
    sh.extend_from_slice(&(ext.len() as u16).to_be_bytes());
    sh.extend_from_slice(&ext);
    let sh = handshake(2, &sh);

    let mut transcript = Transcript::new();
    transcript.push(&ch);
    transcript.push(&sh);
    let keys = crate::schedule::handshake_keys(&shared, &transcript.digest(), SUITE).expect("keys");

    let signer = SigningKey::from_bytes(&[0x17u8; 32].into()).expect("scalar");
    let point = signer.verifying_key().to_encoded_point(false);
    let cert = certificate(point.as_bytes());
    let mut list = Vec::new();
    list.extend_from_slice(&u24(cert.len()));
    list.extend_from_slice(&cert);
    list.extend_from_slice(&[0, 0]);
    let mut cbody = alloc::vec![0u8];
    cbody.extend_from_slice(&u24(list.len()));
    cbody.extend_from_slice(&list);

    let ee = handshake(8, &[0, 0]);
    let cm = handshake(11, &cbody);
    transcript.push(&ee);
    transcript.push(after_ee);
    transcript.push(&cm);
    transcript.push(after_cert);
    let mut blob = alloc::vec![0x20u8; 64];
    blob.extend_from_slice(b"TLS 1.3, server CertificateVerify");
    blob.push(0);
    blob.extend_from_slice(&transcript.digest());
    let sig: Signature = signer.sign(&blob);
    let der = sig.to_der();
    let mut cv = alloc::vec![0x04, 0x03];
    cv.extend_from_slice(&(der.as_bytes().len() as u16).to_be_bytes());
    cv.extend_from_slice(der.as_bytes());
    let cv = handshake(15, &cv);
    transcript.push(&cv);
    let mac = crate::finished_value::finished_value(&keys.server_secret, &transcript.digest()).expect("mac");
    let fin = handshake(20, &mac);
    transcript.push(&fin);
    let app = crate::app_keys::app_keys(&keys, &transcript.digest()).expect("app keys");

    let mut flight = alloc::vec![22, 0x03, 0x03];
    flight.extend_from_slice(&(sh.len() as u16).to_be_bytes());
    flight.extend_from_slice(&sh);
    let msgs = [ee, after_ee.to_vec(), cm, after_cert.to_vec(), cv, fin].concat();
    let sealed = crate::record_seal::seal(SUITE, &keys.server_key, &keys.server_iv, 0, 22, &msgs).expect("seal");
    flight.extend_from_slice(&sealed);
    (flight, app, (keys, transcript))
}
