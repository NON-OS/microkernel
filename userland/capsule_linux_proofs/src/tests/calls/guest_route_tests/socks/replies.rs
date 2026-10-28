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

//! The endpoint's replies byte for byte, and the code each open's errno
//! comes to: RFC 1928's codes for SOCKS5, 90 and 91 for SOCKS4.

use crate::linux::abi::errno::*;
use crate::linux::net::routed::socks::parsed::Refusal;
use crate::linux::net::routed::socks::reply::*;
use crate::linux::net::routed::socks::Ver;

#[test]
fn replies_are_whole_and_name_no_bound_address() {
    assert_eq!(method(true), [5, 0]);
    assert_eq!(method(false), [5, 0xFF]);
    assert_eq!(reply(Ver::Five, SUCCEEDED), [5, 0, 0, 1, 0, 0, 0, 0, 0, 0]);
    assert_eq!(reply(Ver::Five, CONNECTION_REFUSED), [5, 5, 0, 1, 0, 0, 0, 0, 0, 0]);
    assert_eq!(reply(Ver::Four, SUCCEEDED), [0, 90, 0, 0, 0, 0, 0, 0]);
    for code in 1u8..=255 {
        assert_eq!(reply(Ver::Four, code), [0, 91, 0, 0, 0, 0, 0, 0], "code {code}");
        assert_eq!(reply(Ver::Five, code)[1], code);
    }
}

#[test]
fn the_codes_are_rfc_1928_s() {
    let codes = [
        SUCCEEDED,
        GENERAL_FAILURE,
        NOT_ALLOWED,
        NETWORK_UNREACHABLE,
        HOST_UNREACHABLE,
        CONNECTION_REFUSED,
    ];
    assert_eq!(codes, [0, 1, 2, 3, 4, 5]);
    assert_eq!((COMMAND_NOT_SUPPORTED, ADDRESS_TYPE_NOT_SUPPORTED), (7, 8));
    assert_eq!((GRANTED, REJECTED), (90, 91));
    assert_eq!(Refusal::Malformed.code(), GENERAL_FAILURE);
}

#[test]
fn each_errno_an_open_ends_in_has_its_code() {
    for (errno, code) in [
        (EACCES, NOT_ALLOWED),
        (ENETUNREACH, NETWORK_UNREACHABLE),
        (EHOSTUNREACH, HOST_UNREACHABLE),
        (ETIMEDOUT, HOST_UNREACHABLE),
        (ECONNREFUSED, CONNECTION_REFUSED),
        (EIO, GENERAL_FAILURE),
        (ENOBUFS, GENERAL_FAILURE),
        (EINVAL, GENERAL_FAILURE),
    ] {
        assert_eq!(code_of(errno), code, "errno {errno}");
    }
    /* Nothing that failed is ever said as success. */
    assert!((-200i64..200).all(|e| code_of(e) != SUCCEEDED));
}
