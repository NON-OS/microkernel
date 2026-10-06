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


//! The ClientHello: TLS 1.2 only, the two suites, P-256, RSA signatures,
//! the extended master secret, and an empty renegotiation_info that says
//! this client will never renegotiate (RFC 5746 3.4).

extern crate alloc;

use alloc::vec::Vec;

use nonos_libc::crypto_random;

use super::constants::*;
use super::error::Tls12Error;
use super::messages::header;
use super::record::plain;

pub struct Hello {
    pub random: [u8; 32],
    /// The handshake message, for the transcript.
    pub message: Vec<u8>,
    /// The message as a plaintext record, for the wire.
    pub record: Vec<u8>,
}

fn extension(out: &mut Vec<u8>, kind: u16, body: &[u8]) {
    out.extend_from_slice(&kind.to_be_bytes());
    out.extend_from_slice(&(body.len() as u16).to_be_bytes());
    out.extend_from_slice(body);
}

/// A hello naming `sni`. Refused when randomness is unavailable or the name
/// is longer than a hostname can be.
pub fn client_hello(sni: &[u8]) -> Result<Hello, Tls12Error> {
    if sni.is_empty() || sni.len() > 255 {
        return Err(Tls12Error::Malformed);
    }
    let mut random = [0u8; 32];
    if crypto_random(random.as_mut_ptr(), random.len()) != random.len() as i64 {
        return Err(Tls12Error::Crypto);
    }

    let mut ext = Vec::new();
    let mut name = Vec::new();
    name.extend_from_slice(&((sni.len() + 3) as u16).to_be_bytes());
    name.push(0);
    name.extend_from_slice(&(sni.len() as u16).to_be_bytes());
    name.extend_from_slice(sni);
    extension(&mut ext, EXT_SERVER_NAME, &name);
    extension(&mut ext, EXT_EC_POINT_FORMATS, &[1, POINT_UNCOMPRESSED]);
    let mut groups = 2u16.to_be_bytes().to_vec();
    groups.extend_from_slice(&GROUP_P256.to_be_bytes());
    extension(&mut ext, EXT_SUPPORTED_GROUPS, &groups);
    let mut sigs = ((SIGNATURES.len() * 2) as u16).to_be_bytes().to_vec();
    for s in SIGNATURES {
        sigs.extend_from_slice(&s.to_be_bytes());
    }
    extension(&mut ext, EXT_SIGNATURE_ALGORITHMS, &sigs);
    extension(&mut ext, EXT_EXTENDED_MASTER_SECRET, &[]);
    extension(&mut ext, EXT_RENEGOTIATION_INFO, &[0]);

    let mut body = Vec::with_capacity(128 + ext.len());
    body.extend_from_slice(&VERSION.to_be_bytes());
    body.extend_from_slice(&random);
    body.push(0); // no session to resume
    body.extend_from_slice(&((SUITES.len() * 2) as u16).to_be_bytes());
    for s in SUITES {
        body.extend_from_slice(&s.to_be_bytes());
    }
    body.extend_from_slice(&[1, 0]); // compression: null only
    body.extend_from_slice(&(ext.len() as u16).to_be_bytes());
    body.extend_from_slice(&ext);

    let mut message = header(HS_CLIENT_HELLO, body.len()).to_vec();
    message.extend_from_slice(&body);
    let record = plain(CT_HANDSHAKE, &message);
    Ok(Hello { random, message, record })
}
