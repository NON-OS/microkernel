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


//! Which mirror Alpine's packages come from, and the Host line it is asked
//! with: Alpine's CDN by name, so the network the install leaves through
//! resolves it where it leaves, and a mirror given by address is asked as
//! Alpine's CDN.

use crate::install::mirror::{mirror, parse, HOST_LINE};

#[test]
fn the_default_mirror_is_named_not_an_address() {
    /* This crate is built without NONOS_ALPINE_MIRROR, as an image is. */
    let (at, port, host) = mirror();
    assert_eq!((at, port, host), ("dl-cdn.alpinelinux.org", 80, "dl-cdn.alpinelinux.org"));
    assert!(!at.split('.').all(|o| o.parse::<u8>().is_ok()), "a name, not a dotted quad");
}

#[test]
fn a_named_mirror_is_asked_by_its_own_name() {
    assert_eq!(parse("mirror.example.org:8080"), ("mirror.example.org", 8080, "mirror.example.org"));
    assert_eq!(parse("mirror.example.org"), ("mirror.example.org", 80, "mirror.example.org"));
}

#[test]
fn a_mirror_given_by_address_is_asked_as_alpines_cdn() {
    assert_eq!(parse("10.0.2.2:8080"), ("10.0.2.2", 8080, HOST_LINE));
    assert_eq!(parse("151.101.66.132"), ("151.101.66.132", 80, HOST_LINE));
}

#[test]
fn an_unreadable_port_is_eighty() {
    assert_eq!(parse("mirror.example.org:http").1, 80);
    assert_eq!(parse("mirror.example.org:99999").1, 80);
}
