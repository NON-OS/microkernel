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

//! Which certificate in a chain is allowed to have signed the one below.

use crate::cert_is_ca::cert_is_ca;
use crate::cert_valid_now::cert_valid_now;
use crate::example_ca::EXAMPLE_CA;
use crate::example_leaf::EXAMPLE_LEAF;
use crate::relay_cert::RELAY_CERT;

/*
 * Times are packed decimal, YYYYMMDDhhmmss, on both sides of the comparison: the
 * clock reader builds one and the certificate parser builds one, and an integer
 * comparison of that shape orders correctly. Fixed values are used here rather
 * than the real clock, so the suite proves the same thing in a year as it does
 * today.
 *
 * The leaf was served on 2026-09-20 and runs from 2026-07-29 to 2026-10-27.
 */
const INSIDE: u64 = 20260920000000;

#[test]
fn only_the_issuer_is_a_certificate_authority() {
    assert!(cert_is_ca(EXAMPLE_CA), "the intermediate is a CA");
    assert!(!cert_is_ca(EXAMPLE_LEAF), "the leaf is not, and must never sign");
}
#[test]
fn a_relay_certificate_is_not_a_certificate_authority() {
    assert!(!cert_is_ca(RELAY_CERT));
}
#[test]
fn no_prefix_of_a_certificate_passes_a_structural_check() {
    for cut in (0..EXAMPLE_LEAF.len()).step_by(7) {
        let part = &EXAMPLE_LEAF[..cut];
        assert!(!cert_valid_now(part, INSIDE), "validity at {cut}");
        assert!(!cert_is_ca(part), "ca flag at {cut}");
    }
}

/*
 * cert_dns_match is not in the list above; chain_names_forged holds the same
 * property for it. It used to find the subjectAltName by searching for the
 * OID's bytes, which let a cut certificate still claim a name, and let a name
 * planted in a signed public key be believed. It now walks the extension list
 * as these do.
 */
