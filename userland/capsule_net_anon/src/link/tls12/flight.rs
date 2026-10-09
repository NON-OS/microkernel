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


//! Reading the server's first flight in its one allowed order:
//! ServerHello, Certificate, ServerKeyExchange, an optional
//! CertificateRequest, ServerHelloDone. Anything else, in any other place,
//! ends the handshake.

extern crate alloc;

use alloc::vec::Vec;

use nonos_tls::Io;

use super::alert::description;
use super::constants::*;
use super::error::Tls12Error;
use super::gather::gather;
use super::messages::Messages;
use super::record::take;
use super::server::{certificate, certificate_request, key_exchange, server_hello, ServerHello};

pub struct Flight {
    pub hello: ServerHello,
    pub leaf: Vec<u8>,
    pub point: [u8; 65],
    pub cert_request: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Want {
    Hello,
    Certificate,
    KeyExchange,
    RequestOrDone,
    Done,
}

/// The next handshake message from the server, reading records as needed.
/// Only handshake records may arrive before ChangeCipherSpec; an alert
/// ends the handshake with its description.
pub fn next_message<S: Io>(io: &mut S, buf: &mut Vec<u8>, msgs: &mut Messages, hello: bool) -> Result<Vec<u8>, Tls12Error> {
    loop {
        if let Some(message) = msgs.next()? {
            return Ok(message);
        }
        match take(buf, hello)? {
            Some((CT_HANDSHAKE, body)) => msgs.push(&body)?,
            Some((CT_ALERT, body)) => return Err(Tls12Error::PeerAlert(description(&body)?)),
            Some(_) => return Err(Tls12Error::Unexpected),
            None => gather(io, buf)?,
        }
    }
}

pub fn read<S: Io>(io: &mut S, buf: &mut Vec<u8>, transcript: &mut Vec<u8>, client_random: &[u8; 32]) -> Result<Flight, Tls12Error> {
    let mut msgs = Messages::default();
    let mut want = Want::Hello;
    let mut hello = None;
    let mut leaf = None;
    let mut point = None;
    let mut cert_request = false;
    while want != Want::Done {
        let message = next_message(io, buf, &mut msgs, want == Want::Hello)?;
        let (kind, body) = (message[0], &message[4..]);
        want = match (want, kind) {
            (Want::Hello, HS_SERVER_HELLO) => {
                hello = Some(server_hello(body)?);
                Want::Certificate
            }
            (Want::Certificate, HS_CERTIFICATE) => {
                leaf = Some(certificate(body)?);
                Want::KeyExchange
            }
            (Want::KeyExchange, HS_SERVER_KEY_EXCHANGE) => {
                let (Some(h), Some(l)) = (hello.as_ref(), leaf.as_ref()) else {
                    return Err(Tls12Error::Unexpected);
                };
                let ske = key_exchange(body)?;
                verify(l, client_random, &h.random, &ske)?;
                point = Some(ske.point);
                Want::RequestOrDone
            }
            (Want::RequestOrDone, HS_CERTIFICATE_REQUEST) if !cert_request => {
                certificate_request(body)?;
                cert_request = true;
                Want::RequestOrDone
            }
            (Want::RequestOrDone, HS_SERVER_HELLO_DONE) if body.is_empty() => Want::Done,
            _ => return Err(Tls12Error::Unexpected),
        };
        transcript.extend_from_slice(&message);
    }
    if !msgs.is_empty() {
        return Err(Tls12Error::Unexpected);
    }
    match (hello, leaf, point) {
        (Some(hello), Some(leaf), Some(point)) => Ok(Flight { hello, leaf, point, cert_request }),
        _ => Err(Tls12Error::Unexpected),
    }
}

/// The ServerKeyExchange signature over client random, server random and
/// the ECDHE parameters, under the leaf certificate's RSA key, by the crypto
/// pool's RSA verify. This is what proves the relay holds the key of the
/// certificate the CERTS cell will vouch for.
fn verify(leaf: &[u8], client_random: &[u8; 32], server_random: &[u8; 32], ske: &super::server::KeyExchange) -> Result<(), Tls12Error> {
    let spki = super::spki::spki(leaf).ok_or(Tls12Error::Malformed)?;
    let mut signed = Vec::with_capacity(64 + ske.params.len());
    signed.extend_from_slice(client_random);
    signed.extend_from_slice(server_random);
    signed.extend_from_slice(&ske.params);
    // verify_rsa's scheme: 0 PKCS#1 v1.5, 1 PSS; its hash: 0 SHA-256, 1 SHA-384.
    let (scheme, hashid) = match ske.scheme {
        SIG_RSA_PKCS1_SHA256 => (0, 0),
        SIG_RSA_PSS_SHA256 => (1, 0),
        SIG_RSA_PKCS1_SHA384 => (0, 1),
        SIG_RSA_PSS_SHA384 => (1, 1),
        _ => return Err(Tls12Error::Unoffered),
    };
    let digest: Vec<u8> = if hashid == 0 {
        crate::crypto::sha256(&signed).map_err(|_| Tls12Error::Crypto)?.to_vec()
    } else {
        crate::crypto::sha384::sha384(&[&signed]).to_vec()
    };
    if nonos_tls::verify_rsa(scheme, hashid, spki, &ske.signature, &digest) {
        Ok(())
    } else {
        Err(Tls12Error::Signature)
    }
}
