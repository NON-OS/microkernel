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

//! The RELAY_BEGIN body, against tor-spec section 6.2.

extern crate alloc;

use crate::stream::begin::body;

#[test]
fn the_body_is_host_colon_port_nul_then_flags() {
    let built = body(b"example.com", 443).expect("fits");
    assert_eq!(&built[..15], b"example.com:443");
    assert_eq!(built[15], 0, "the string is nul terminated");
    assert_eq!(&built[16..], &[0, 0, 0, 1], "IPV6_OK and nothing else");
}

#[test]
fn a_port_is_written_without_padding() {
    assert_eq!(&body(b"h", 80).expect("fits")[..4], b"h:80");
    assert_eq!(&body(b"h", 8).expect("fits")[..3], b"h:8");
    assert_eq!(&body(b"h", 65535).expect("fits")[..7], b"h:65535");
}

#[test]
fn an_overlong_host_is_refused() {
    let long = alloc::vec![b'a'; 500];
    assert!(body(&long, 443).is_none());
    let just_fits = alloc::vec![b'a'; 480];
    assert!(body(&just_fits, 443).is_some());
}
