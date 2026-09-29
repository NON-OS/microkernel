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
//! Authority cert.

use crate::directory::verify::parse;

/*
 * Served by 49.13.145.234 for v3 identity 88F29CB5FE86A688E31990A3B20BD562D0C089E1
 * on 2026-09-19. A real document rather than a written one, because what is being
 * checked is that the fields are where an authority actually puts them.
 */
const CERT: &[u8] = include_bytes!("../../vectors/authority-cert.txt");

#[test]
fn a_live_certificate_carries_both_keys_and_its_certification() {
    let cert = parse(CERT).expect("a real authority certificate parses");
    assert!(!cert.identity_pkcs1.is_empty(), "the identity key, whose SHA-1 is the v3 identity");
    assert!(!cert.signing_pkcs1.is_empty(), "the signing key, which signs consensuses");
    assert!(!cert.certification.is_empty(), "the identity key's signature over the certificate");
    let (from, to) = cert.signed;
    assert!(to > from && to <= CERT.len(), "the certified span lies inside the document");
}
/*
 * `dir-key-expires 2027-06-27 15:46:28` on this certificate. The epoch was
 * computed outside this crate, so a calendar mistake in the capsule's own date
 * arithmetic shows up here rather than agreeing with itself.
 */
#[test]
fn the_expiry_reads_as_the_epoch_the_calendar_gives() {
    let cert = parse(CERT).expect("parses");
    assert_eq!(cert.expires, 1_814_111_188, "2027-06-27 15:46:28 UTC");
}
