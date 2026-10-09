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


//! The recordings, and what is in them, read independently of the client.

use std::string::String;
use std::vec::Vec;

pub struct Case {
    pub name: &'static str,
    pub server: &'static [u8],
    pub client: &'static [u8],
    pub outcome: &'static str,
}

macro_rules! case {
    ($name:literal) => {
        Case {
            name: $name,
            server: include_bytes!(concat!("../../vectors/", $name, ".server")),
            client: include_bytes!(concat!("../../vectors/", $name, ".client")),
            outcome: include_str!(concat!("../../vectors/", $name, ".outcome")),
        }
    };
}

pub fn all() -> Vec<Case> {
    std::vec![
        case!("chacha_certreq"),
        case!("aes_gcm_renegotiate"),
        case!("pkcs1_sha384"),
        case!("pkcs1_sha256"),
        case!("fragmented"),
        case!("no_ems"),
        case!("close_notify"),
        case!("downgrade"),
        case!("no_shared_suite"),
    ]
}

pub fn get(name: &str) -> Case {
    all().into_iter().find(|c| c.name == name).expect("a recorded case")
}

/// The records in `bytes`: type, body, and where each starts.
pub fn records(bytes: &[u8]) -> Vec<(u8, &[u8], usize)> {
    let mut out = Vec::new();
    let mut at = 0;
    while at + 5 <= bytes.len() {
        let len = usize::from(u16::from_be_bytes([bytes[at + 3], bytes[at + 4]]));
        let end = (at + 5 + len).min(bytes.len());
        out.push((bytes[at], &bytes[at + 5..end], at));
        at = end;
    }
    out
}

/// The plaintext handshake messages before ChangeCipherSpec: type and body.
pub fn handshake(bytes: &[u8]) -> Vec<(u8, Vec<u8>)> {
    let mut stream = Vec::new();
    for (kind, body, _) in records(bytes) {
        if kind == 20 {
            break;
        }
        if kind == 22 {
            stream.extend_from_slice(body);
        }
    }
    let mut out = Vec::new();
    let mut at = 0;
    while at + 4 <= stream.len() {
        let len = usize::from(stream[at + 1]) << 16 | usize::from(stream[at + 2]) << 8 | usize::from(stream[at + 3]);
        out.push((stream[at], stream[at + 4..at + 4 + len].to_vec()));
        at += 4 + len;
    }
    out
}

/// The server's chosen suite and extension types, from its ServerHello.
pub fn server_hello(bytes: &[u8]) -> (u16, [u8; 32], Vec<u16>) {
    let (_, body) = handshake(bytes).into_iter().find(|(k, _)| *k == 2).expect("a ServerHello");
    let mut random = [0u8; 32];
    random.copy_from_slice(&body[2..34]);
    let sid = usize::from(body[34]);
    let at = 35 + sid;
    let suite = u16::from_be_bytes([body[at], body[at + 1]]);
    let mut exts = Vec::new();
    let mut e = at + 3 + 2;
    while e + 4 <= body.len() {
        exts.push(u16::from_be_bytes([body[e], body[e + 1]]));
        e += 4 + usize::from(u16::from_be_bytes([body[e + 2], body[e + 3]]));
    }
    (suite, random, exts)
}

/// The ServerKeyExchange's signature scheme.
pub fn signature_scheme(bytes: &[u8]) -> u16 {
    let (_, body) = handshake(bytes).into_iter().find(|(k, _)| *k == 12).expect("a ServerKeyExchange");
    u16::from_be_bytes([body[69], body[70]])
}

/// Where the server's Finished record ends: the first record after its
/// ChangeCipherSpec.
pub fn finished_end(bytes: &[u8]) -> usize {
    let recs = records(bytes);
    let ccs = recs.iter().position(|(k, _, _)| *k == 20).expect("a ChangeCipherSpec");
    let (_, body, at) = recs[ccs + 1];
    at + 5 + body.len()
}

pub fn outcome_line(case: &Case) -> String {
    case.outcome.trim_end().into()
}
