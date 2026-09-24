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

//! The public key span inside a certificate.

use crate::cert_spki::cert_spki;
use crate::relay_cert::{RELAY_CERT, SPKI_AT, SPKI_LEN};

/*
 * Every link handshake in the anon transport ends up here: the key this returns
 * is the key the CertificateVerify signature is checked against. Return the
 * wrong bytes and the signature cannot verify, the guard is dropped as
 * unreachable, and the log blames the relay.
 *
 * The expected span comes from openssl at capture time, so this measures the
 * parser against another implementation rather than against itself.
 */
#[test]
fn the_public_key_is_the_span_openssl_reports() {
    let spki = cert_spki(RELAY_CERT).expect("a relay certificate must yield a key");
    assert_eq!(spki.len(), SPKI_LEN, "length must match openssl");
    assert_eq!(spki, &RELAY_CERT[SPKI_AT..SPKI_AT + SPKI_LEN], "and so must the bytes");
}
#[test]
fn the_span_is_a_complete_spki_structure() {
    let spki = cert_spki(RELAY_CERT).expect("key");
    assert_eq!(spki[0], 0x30, "a SEQUENCE, not the contents of one");
    let (tag, val, end) = crate::der_tlv::der_tlv(spki, 0).expect("well formed");
    assert_eq!(tag, 0x30);
    assert_eq!(end, spki.len(), "the structure ends exactly where the span does");
    assert!(val < end);
}
