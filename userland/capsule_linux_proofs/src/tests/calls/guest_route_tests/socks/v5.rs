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

//! SOCKS5: every greeting and request form, and every truncation.

use super::v5_request;
use crate::linux::net::routed::socks::parsed::Parsed;
use crate::linux::net::routed::socks::v5::{greeting, request};
use crate::linux::net::routed::socks::{Connect, To};

#[test]
fn a_greeting_offering_no_authentication_is_taken_whole() {
    assert_eq!(greeting(&[5, 1, 0]), Parsed::Done(true, 3));
    assert_eq!(greeting(&[5, 3, 2, 1, 0, 9]), Parsed::Done(true, 5));
    assert_eq!(greeting(&[5, 2, 2, 1]), Parsed::Done(false, 4), "user/password, GSSAPI only");
    assert_eq!(greeting(&[5, 0]), Parsed::Done(false, 2), "no method at all");
    for cut in 0..3 {
        assert_eq!(greeting(&[5, 1, 0][..cut]), Parsed::Need, "cut at {cut}");
    }
    assert_eq!(greeting(&[5, 255]), Parsed::Need);
    for ver in (0u8..=255).filter(|&v| v != 5) {
        assert_eq!(greeting(&[ver, 1, 0]), Parsed::Bad);
    }
}

#[test]
fn a_connect_by_name_keeps_the_name_and_by_address_the_address() {
    for (to, port) in [
        (To::Name(b"node.example.org".to_vec()), 18089),
        (To::Name(vec![b'a'; 255]), 1),
        (To::V4([198, 51, 100, 7]), 18081),
        (To::V4([100, 64, 0, 3]), 443),
    ] {
        let req = v5_request(1, &to, port);
        assert_eq!(request(&req), Parsed::Done(Connect { to: to.clone(), port }, req.len()));
        for cut in 0..req.len() {
            assert_eq!(request(&req[..cut]), Parsed::Need, "{to:?} cut at {cut}");
        }
        let mut more = req.clone();
        more.extend_from_slice(b"GET / HTTP/1.1");
        assert_eq!(request(&more), Parsed::Done(Connect { to, port }, req.len()), "pipelined");
    }
}
