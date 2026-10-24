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

//! The names a peer sends: user, role and host, each of one fixed shape.

use nsh_wire::names::{host, role, user};
use nsh_wire::WireError::Value;

#[test]
fn a_user_name_is_lowercase_led_and_at_most_thirty_two() {
    for ok in [&b"ek"[..], b"a", b"ops.night-2_b", &[b'a'; 32]] {
        assert_eq!(user(ok), Ok(()), "{ok:?}");
    }
    for bad in [&b""[..], b"Ek", b"2ek", b".ek", b"e k", b"e\x1bk", b"\xC3\xA9", &[b'a'; 33]] {
        assert_eq!(user(bad), Err(Value), "{bad:?}");
    }
}

#[test]
fn a_role_is_lowercase_letters_digits_and_dashes() {
    for ok in [&b"operator"[..], b"deploy-2", &[b'r'; 16]] {
        assert_eq!(role(ok), Ok(()), "{ok:?}");
    }
    for bad in [&b""[..], b"Operator", b"-ops", b"ops.x", b"ops_x", &[b'r'; 17]] {
        assert_eq!(role(bad), Err(Value), "{bad:?}");
    }
}

#[test]
fn a_host_name_is_lowercase_dns_labels() {
    let long_label = [b'a'; 63];
    for ok in [&b"db-1.eu.example"[..], b"x", b"1.2", &long_label] {
        assert_eq!(host(ok), Ok(()), "{ok:?}");
    }
    let too_long = [&[b'a'; 63][..], b".", &[b'b'; 63], b".", &[b'c'; 63], b".", &[b'd'; 62]];
    assert_eq!(too_long.concat().len(), 254);
    for bad in [
        &b""[..],
        b"DB.example",
        b"db..example",
        b".db",
        b"db.",
        b"-db.x",
        b"db-.x",
        b"db_1.x",
        b"db 1",
        &[b'a'; 64],
    ] {
        assert_eq!(host(bad), Err(Value), "{bad:?}");
    }
    assert_eq!(host(&too_long.concat()), Err(Value));
    assert_eq!(host(&too_long.concat()[1..]), Ok(()));
}
