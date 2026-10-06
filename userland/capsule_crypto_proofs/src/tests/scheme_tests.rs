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

//! That adding scheme 2 did not widen anything else.

extern crate alloc;

use crate::cert::{parse, CERT};
use crate::rsa_scheme::{digest_len, verify};
use crate::spki::wrap_pkcs1;

#[test]
fn the_prefixed_and_pss_schemes_keep_their_lengths() {
    for scheme in [0u8, 1] {
        assert_eq!(digest_len(scheme, 0), Some(32));
        assert_eq!(digest_len(scheme, 1), Some(48));
        assert_eq!(digest_len(scheme, 2), Some(64));
    }
}

#[test]
fn a_twenty_byte_digest_is_reachable_under_the_pkcs1_schemes_only() {
    assert_eq!(digest_len(2, 3), Some(20));
    assert_eq!(digest_len(0, 3), Some(20));
    assert_eq!(digest_len(1, 3), None);
    assert_eq!(digest_len(3, 3), None, "there is no scheme 3");
}

/// Scheme 2 still honours the other digest lengths, since a consensus signature
#[test]
fn scheme_two_accepts_the_longer_lengths_too() {
    assert_eq!(digest_len(2, 0), Some(32));
    assert_eq!(digest_len(2, 1), Some(48));
    assert_eq!(digest_len(2, 2), Some(64));
}

#[test]
fn an_unknown_scheme_is_refused() {
    let cert = parse(CERT);
    let spki = wrap_pkcs1(&cert.identity_pkcs1);
    assert_eq!(verify(9, 3, &spki, &cert.signature, &cert.digest), None);
}

#[test]
fn a_bare_pkcs1_key_is_refused_as_an_argument() {
    let cert = parse(CERT);
    let verdict = verify(2, 3, &cert.identity_pkcs1, &cert.signature, &cert.digest);
    assert_eq!(verdict, None, "the pool takes SubjectPublicKeyInfo, not PKCS#1");
}
