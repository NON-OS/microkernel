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

//! A certificate body longer than one pool request still verifies.

use crate::cert_spki::cert_spki;
use crate::cert_tbs::cert_tbs;
use crate::fixtures::certs::{P384_AUTHORITY, P384_MANY_NAMES};
use crate::verify_link::verify_link;

#[test]
fn a_p384_signature_over_a_long_body_verifies() {
    let tbs = cert_tbs(P384_MANY_NAMES).expect("tbs");
    assert!(tbs.len() > 1516, "longer than the old 1,536 byte request allowed");
    let spki = cert_spki(P384_AUTHORITY).expect("authority key");
    nonos_libc::reset();
    assert!(verify_link(P384_MANY_NAMES, spki));
    let c = nonos_libc::counts();
    assert_eq!((c.p384, c.sha384), (1, 0), "hashed here, signature checked by the pool");
}

#[test]
fn the_same_body_changed_by_one_name_does_not() {
    let tbs = cert_tbs(P384_MANY_NAMES).expect("tbs");
    let start = tbs.as_ptr() as usize - P384_MANY_NAMES.as_ptr() as usize;
    let name = tbs.windows(5).rposition(|w| w == b"proof").expect("a name");
    let mut forged = P384_MANY_NAMES.to_vec();
    forged[start + name] ^= 0x20;
    let spki = cert_spki(P384_AUTHORITY).expect("authority key");
    assert!(!verify_link(&forged, spki));
}
