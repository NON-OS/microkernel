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

//! SOCKS4 and SOCKS4a: the address, the user id read past, the 4a name kept a name.

use super::{name, v4a_request};
use crate::linux::net::routed::socks::parsed::{Parsed, Refusal};
use crate::linux::net::routed::socks::v4::request;
use crate::linux::net::routed::socks::{Connect, To};

#[test]
fn socks4a_as_monero_sends_it_carries_the_name() {
    let req = v4a_request(b"node.moneroworld.com", 18089);
    assert_eq!(request(&req), Parsed::Done(name("node.moneroworld.com", 18089), req.len()));
    for cut in 0..req.len() {
        assert_eq!(request(&req[..cut]), Parsed::Need, "cut at {cut}");
    }
    /* Any 0.0.0.x but 0.0.0.0 says a name follows, and any user id is passed over. */
    for x in [1u8, 7, 255] {
        let mut r = vec![4, 1, 0x46, 0xA9, 0, 0, 0, x];
        r.extend_from_slice(b"monero\0n.example\0");
        assert_eq!(request(&r), Parsed::Done(name("n.example", 18089), r.len()), "0.0.0.{x}");
    }
}

#[test]
fn socks4_carries_the_address_and_ignores_the_user_id() {
    for user in [&b""[..], b"u", &[b'x'; 300]] {
        let mut req = vec![4, 1, 0x1F, 0x90, 198, 51, 100, 7];
        req.extend_from_slice(user);
        req.push(0);
        let want = Connect { to: To::V4([198, 51, 100, 7]), port: 8080 };
        assert_eq!(request(&req), Parsed::Done(want.clone(), req.len()));
        for cut in 0..req.len() {
            assert_eq!(request(&req[..cut]), Parsed::Need, "cut at {cut}");
        }
        let mut more = req.clone();
        more.extend_from_slice(b"\x16\x03\x01");
        assert_eq!(request(&more), Parsed::Done(want, req.len()), "pipelined");
    }
}

#[test]
fn bind_and_every_other_command_are_refused() {
    for cmd in (0u8..=255).filter(|&c| c != 1) {
        assert_eq!(request(&[4, cmd]), Parsed::Refused(Refusal::Command), "cmd {cmd}");
    }
}

#[test]
fn a_request_no_client_sends_is_refused_or_closed() {
    let bad = Parsed::Refused(Refusal::Malformed);
    assert_eq!(request(&[4, 1, 0, 0, 1, 2, 3, 4, 0]), bad, "port 0");
    assert_eq!(request(&[4, 1, 0, 80, 0, 0, 0, 0, 0]), bad, "0.0.0.0");
    assert_eq!(request(&[4, 1, 0, 80, 0, 0, 0, 1, 0, 0]), bad, "a 4a name that is empty");
    assert_eq!(request(&[4, 1, 0, 80, 0, 0, 0, 1, 0, b'a', b' ', 0]), bad, "a space");
    let long = v4a_request(&[b'a'; 256], 80);
    assert_eq!(request(&long), bad, "a name past 255 bytes");
    for ver in (0u8..=255).filter(|&v| v != 4) {
        assert_eq!(request(&[ver, 1, 0, 80, 1, 2, 3, 4, 0]), Parsed::Bad, "version {ver}");
    }
}
