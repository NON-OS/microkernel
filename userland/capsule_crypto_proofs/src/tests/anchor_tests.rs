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

//! That the vector really is the authority the capsule trusts.

extern crate alloc;

use alloc::string::String;
use core::fmt::Write;
use sha1::{Digest, Sha1};

use crate::cert::{parse, CERT};

const V3IDENT: &str = "88f29cb5fe86a688e31990a3b20bd562d0c089e1";

fn hex(bytes: &[u8]) -> String {
    let mut out = String::new();
    for byte in bytes {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

/*
 * The fingerprint is taken over the PKCS#1 RSAPublicKey DER, not over the
 * SubjectPublicKeyInfo the verifier wants. Hashing the wrong form gives a
 * fingerprint that matches no authority, so both forms of the same key are
 * needed: the inner bytes to check the identity, the wrapped bytes to check the
 * signature.
 */
#[test]
fn the_identity_key_hashes_to_the_hardcoded_v3_identity() {
    let cert = parse(CERT);
    assert_eq!(hex(&Sha1::digest(&cert.identity_pkcs1)), V3IDENT);
    assert_eq!(cert.digest.len(), 20, "the span digest is SHA-1");
}
