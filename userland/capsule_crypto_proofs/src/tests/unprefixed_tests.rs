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

//! Scheme 2 against a real authority certificate signature.

use crate::cert::{parse, CERT};
use crate::rsa_scheme::verify;
use crate::spki::wrap_pkcs1;

#[test]
fn a_real_authority_certificate_verifies_under_scheme_two() {
    let cert = parse(CERT);
    let spki = wrap_pkcs1(&cert.identity_pkcs1);
    assert_eq!(
        verify(2, 3, &spki, &cert.signature, &cert.digest),
        Some(true),
        "the certificate the authority served did not verify"
    );
}

#[test]
fn the_same_signature_is_refused_by_the_prefixed_schemes() {
    let cert = parse(CERT);
    let spki = wrap_pkcs1(&cert.identity_pkcs1);
    /*
     * A 20 byte digest has no prefixed home, so this is a bad argument rather
     * than a failed signature, which is itself the point.
     */
    assert_eq!(verify(0, 3, &spki, &cert.signature, &cert.digest), None);
    // Offered at a SHA-256 length, the digest no longer fits.
    assert_eq!(verify(0, 0, &spki, &cert.signature, &cert.digest), None);
}

#[test]
fn a_tampered_digest_is_refused() {
    let cert = parse(CERT);
    let spki = wrap_pkcs1(&cert.identity_pkcs1);
    let mut digest = cert.digest.clone();
    digest[0] ^= 0x01;
    assert_eq!(verify(2, 3, &spki, &cert.signature, &digest), Some(false));
}
