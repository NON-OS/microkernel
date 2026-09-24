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

//! Whether a certificate was issued for the host being reached.

use crate::cert_dns_match::matches;
use crate::example_leaf::EXAMPLE_LEAF;

#[test]
fn the_hostname_must_match_the_leaf() {
    assert!(matches(EXAMPLE_LEAF, b"example.com"));
    assert!(!matches(EXAMPLE_LEAF, b"evil.com"));
    assert!(!matches(EXAMPLE_LEAF, b""), "an empty host matches nothing");
}
#[test]
fn a_name_that_merely_contains_the_certificate_name_is_refused() {
    for host in [
        b"notexample.com".as_slice(),
        b"example.com.evil.com",
        b"example.como",
        b"xample.com",
        b"example.co",
    ] {
        let name = core::str::from_utf8(host).unwrap_or("?");
        assert!(!matches(EXAMPLE_LEAF, host), "{name} is not example.com");
    }
}
