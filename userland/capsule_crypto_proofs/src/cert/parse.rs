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

//! The signed span, the identity key and the signature of a certificate.

extern crate alloc;

use alloc::vec::Vec;
use sha1::{Digest, Sha1};

use super::find::find;
use super::pem::pem;

/// A certificate served by the Anyone authority at 49.13.145.234:9230 on
/// 2026-09-18, byte for byte as it arrived.
pub const CERT: &[u8] = include_bytes!("../../vectors/anyone-authority-cert.txt");

/// The three pieces a signature check needs from a certificate.
pub struct Certificate {
    /// SHA-1 over the signed span, which is what the signature commits to.
    pub digest: Vec<u8>,
    /// The identity key as PKCS#1 RSAPublicKey DER, the form the document
    /// carries and the form the fingerprint is taken over.
    pub identity_pkcs1: Vec<u8>,
    pub signature: Vec<u8>,
}

/*
 * The signed span runs from "dir-key-certificate-version" up to and including
 * the first newline after "\ndir-key-certification". Read from
 * router_get_hash_impl in the fork's authcert_parse.c, which passes '\n' as the
 * terminator where the consensus boundary passes a space. Using the consensus
 * rule here yields a digest that never matches.
 */
/// Parse the vector. Panics on a malformed certificate, which is right for a
/// test over a constant.
pub fn parse(body: &[u8]) -> Certificate {
    let start = find(body, b"dir-key-certificate-version", 0).expect("version line");
    let marker = find(body, b"\ndir-key-certification", start).expect("certification line");
    let after = marker + b"\ndir-key-certification".len();
    let end = find(body, b"\n", after).expect("newline after the keyword") + 1;

    let from = find(body, b"dir-identity-key", 0).expect("identity key");
    let to = find(body, b"dir-signing-key", 0).expect("signing key");
    Certificate {
        digest: Sha1::digest(&body[start..end]).to_vec(),
        identity_pkcs1: pem(&body[from..to], b"RSA PUBLIC KEY"),
        signature: pem(&body[marker..], b"SIGNATURE"),
    }
}
