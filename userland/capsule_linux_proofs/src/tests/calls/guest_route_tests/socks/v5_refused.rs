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

//! SOCKS5 refusals, each by its own reason: a command but CONNECT (BIND,
//! UDP ASSOCIATE) is 7, IPv6 or any address type but IPv4 and a name is 8,
//! and a request no client sends is a general failure or no reply at all.

use super::{name, v5_request};
use crate::linux::net::routed::socks::parsed::{Parsed, Refusal};
use crate::linux::net::routed::socks::v5::request;
use crate::linux::net::routed::socks::To;

#[test]
fn bind_udp_and_every_other_command_are_refused_as_commands() {
    for cmd in (0u8..=255).filter(|&c| c != 1) {
        let req = v5_request(cmd, &To::V4([1, 2, 3, 4]), 80);
        assert_eq!(request(&req[..2]), Parsed::Refused(Refusal::Command), "cmd {cmd}");
        assert_eq!(request(&req), Parsed::Refused(Refusal::Command), "cmd {cmd}");
        assert_eq!(Refusal::Command.code(), 7);
    }
}

#[test]
fn ipv6_and_unknown_address_types_are_refused_before_their_address() {
    for atyp in (0u8..=255).filter(|&a| a != 1 && a != 3) {
        assert_eq!(request(&[5, 1, 0, atyp]), Parsed::Refused(Refusal::AddressType), "{atyp}");
    }
    let mut v6 = vec![5, 1, 0, 4];
    v6.extend_from_slice(&[0x20, 1, 0xd, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 80]);
    assert_eq!(request(&v6), Parsed::Refused(Refusal::AddressType));
    assert_eq!(Refusal::AddressType.code(), 8);
}

#[test]
fn a_request_no_client_sends_is_refused_or_closed() {
    let bad = Parsed::Refused(Refusal::Malformed);
    assert_eq!(request(&v5_request(1, &To::Name(Vec::new()), 80)), bad, "an empty name");
    assert_eq!(request(&v5_request(1, &To::Name(b"a b".to_vec()), 80)), bad, "a space");
    assert_eq!(request(&v5_request(1, &To::Name(b"h:1".to_vec()), 80)), bad, "a colon");
    assert_eq!(request(&v5_request(1, &To::Name(vec![0xC3, 0xA9]), 80)), bad, "not ASCII");
    assert_eq!(request(&v5_request(1, &To::V4([0; 4]), 80)), bad, "0.0.0.0");
    assert_eq!(request(&v5_request(1, &To::V4([1, 2, 3, 4]), 0)), bad, "port 0");
    assert_eq!(request(&v5_request(1, &name("x", 1).to, 0)), bad);
    for ver in (0u8..=255).filter(|&v| v != 5) {
        assert_eq!(request(&[ver, 1, 0, 1, 1, 2, 3, 4, 0, 80]), Parsed::Bad, "version {ver}");
    }
}
