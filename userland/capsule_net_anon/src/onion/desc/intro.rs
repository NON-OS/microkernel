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


//! The introduction points in a descriptor's inner layer
//! (decode_introduction_point in the fork).

extern crate alloc;

use alloc::vec::Vec;

use crate::directory::base64::decode;
use crate::directory::consensus::object_after;
use crate::directory::lines::{arg, lines};

use super::super::cert::{check, TYPE_INTRO_AUTH, TYPE_INTRO_ENC};

/// HS_CONFIG_V3_MAX_INTRO_POINTS: a service lists at most twenty.
pub const INTRO_MAX: usize = 20;

/* Link specifier types, src/trunnel/link_specifier.trunnel. */
const SPEC_IPV4: u8 = 0;
const SPEC_LEGACY_ID: u8 = 2;
const SPEC_ED25519_ID: u8 = 3;

/// One introduction point: where it is, its ntor onion key for the circuit
/// to it, and the two service keys the INTRODUCE1 cell is made with.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntroPoint {
    pub address: [u8; 4],
    pub port: u16,
    pub rsa_identity: [u8; 20],
    pub ed25519_identity: [u8; 32],
    /// The relay's ntor key, for extending a circuit to it.
    pub onion_key: [u8; 32],
    /// The service's auth key at this point, named in INTRODUCE1.
    pub auth_key: [u8; 32],
    /// The service's X25519 key the INTRODUCE1 payload is encrypted to.
    pub enc_key: [u8; 32],
}

/// Every introduction point in `layer` that parses and whose two
/// certificates verify under `signing_key`. A point missing an IPv4
/// address, an RSA or an Ed25519 identity is skipped: this client extends
/// with all three, as every relay accepts.
pub fn intro_points(layer: &[u8], signing_key: &[u8; 32], now: u64) -> Vec<IntroPoint> {
    let starts: Vec<usize> =
        lines(layer).filter(|l| l.keyword == b"introduction-point").map(|l| l.at).collect();
    let mut out = Vec::new();
    for (index, start) in starts.iter().enumerate() {
        let end = starts.get(index + 1).copied().unwrap_or(layer.len());
        if let Some(point) = one(layer, *start, end, signing_key, now) {
            out.push(point);
        }
        if out.len() == INTRO_MAX {
            break;
        }
    }
    out
}

fn one(layer: &[u8], start: usize, end: usize, signing_key: &[u8; 32], now: u64) -> Option<IntroPoint> {
    let chunk = &layer[start..end];
    let mut specs = None;
    let mut onion_key = None;
    let mut auth_cert = None;
    let mut enc_key = None;
    let mut enc_cert = None;
    for line in lines(chunk) {
        match line.keyword {
            b"introduction-point" => specs = arg(line.rest, 0).and_then(decode),
            b"onion-key" if arg(line.rest, 0) == Some(b"ntor") => onion_key = key32(arg(line.rest, 1)),
            b"auth-key" => auth_cert = object_after(layer, start + line.at),
            b"enc-key" if arg(line.rest, 0) == Some(b"ntor") => enc_key = key32(arg(line.rest, 1)),
            b"enc-key-cert" => enc_cert = object_after(layer, start + line.at),
            _ => {}
        }
    }
    let (auth_key, _) = check(&auth_cert?, TYPE_INTRO_AUTH, Some(signing_key), now)?;
    check(&enc_cert?, TYPE_INTRO_ENC, Some(signing_key), now)?;
    let (address, port, rsa_identity, ed25519_identity) = link_specifiers(&specs?)?;
    Some(IntroPoint {
        address,
        port,
        rsa_identity,
        ed25519_identity,
        onion_key: onion_key?,
        auth_key,
        enc_key: enc_key?,
    })
}

fn key32(text: Option<&[u8]>) -> Option<[u8; 32]> {
    decode(text?)?.try_into().ok()
}

/// Where a relay is and who it is: IPv4 address, ORPort, RSA identity and
/// Ed25519 identity.
pub type LinkSpecifiers = ([u8; 4], u16, [u8; 20], [u8; 32]);

/// NSPEC, then each specifier as type, length, body. Unknown types are
/// skipped, as the trunnel parser keeps them unread.
pub fn link_specifiers(raw: &[u8]) -> Option<LinkSpecifiers> {
    let (&count, mut rest) = raw.split_first()?;
    let mut ipv4 = None;
    let mut rsa = None;
    let mut ed = None;
    for _ in 0..count {
        let kind = *rest.first()?;
        let len = *rest.get(1)? as usize;
        let body = rest.get(2..2 + len)?;
        match (kind, len) {
            (SPEC_IPV4, 6) => {
                let port = u16::from_be_bytes([body[4], body[5]]);
                ipv4 = (port != 0).then(|| ([body[0], body[1], body[2], body[3]], port));
            }
            (SPEC_LEGACY_ID, 20) => rsa = body.try_into().ok(),
            (SPEC_ED25519_ID, 32) => ed = body.try_into().ok(),
            _ => {}
        }
        rest = &rest[2 + len..];
    }
    let (address, port) = ipv4?;
    Some((address, port, rsa?, ed?))
}
