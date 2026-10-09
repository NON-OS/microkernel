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


//! Scheme 0 at SHA-1: a signature made over a DigestInfo, which is how an
//! Alpine package index is signed.

use super::sha1_vector::{DIGEST, SIG, SPKI};
use crate::rsa_scheme::verify;

#[test]
fn a_prefixed_sha1_signature_verifies_under_scheme_zero() {
    assert_eq!(verify(0, 3, &SPKI, &SIG, &DIGEST), Some(true));
}

#[test]
fn a_tampered_digest_or_signature_is_refused() {
    let mut digest = DIGEST;
    digest[19] ^= 0x01;
    assert_eq!(verify(0, 3, &SPKI, &SIG, &digest), Some(false));
    let mut sig = SIG;
    sig[0] ^= 0x01;
    assert_eq!(verify(0, 3, &SPKI, &sig, &DIGEST), Some(false));
}

#[test]
fn the_prefixed_signature_is_not_an_unprefixed_one() {
    /*
     * Scheme 2 expects the bare digest in the padded block, so a DigestInfo
     * there is a different block and must not verify.
     */
    assert_eq!(verify(2, 3, &SPKI, &SIG, &DIGEST), Some(false));
}

#[test]
fn pss_is_never_offered_at_sha1() {
    assert_eq!(verify(1, 3, &SPKI, &SIG, &DIGEST), None);
}
