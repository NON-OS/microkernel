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

//! Certificates a key must not be read out of.

use crate::cert_spki::cert_spki;
use crate::relay_cert::RELAY_CERT;

#[test]
fn no_prefix_of_a_certificate_yields_a_key() {
    for cut in 0..RELAY_CERT.len() {
        assert!(
            cert_spki(&RELAY_CERT[..cut]).is_none(),
            "a certificate cut at {cut} bytes is not a certificate"
        );
    }
    assert!(cert_spki(RELAY_CERT).is_some(), "and the whole one still parses");
}
#[test]
fn a_corrupted_length_is_refused() {
    let mut broken = RELAY_CERT.to_vec();
    broken[2] = broken[2].wrapping_add(1);
    assert!(cert_spki(&broken).is_none(), "an outer length that overruns is refused");
}
#[test]
fn trailing_bytes_are_refused() {
    let mut padded = RELAY_CERT.to_vec();
    padded.push(0x00);
    assert!(cert_spki(&padded).is_none(), "the certificate must end where the buffer does");
}
