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

//! The SOCKS5 the client speaks: the host travels by name, and the replies
//! are read only as far as they have arrived.

use crate::refusal::Proxy;
use crate::socks::{connect_reply, connect_request, method_reply, Parsed, GREETING};

#[test]
fn the_greeting_offers_no_authentication_only() {
    assert_eq!(GREETING, [5, 1, 0]);
}

#[test]
fn the_host_goes_by_name_for_the_exit_to_resolve() {
    let ask = connect_request("ethereum-rpc.publicnode.com", 443).unwrap_or_default();
    let mut want = vec![5, 1, 0, 3, 27];
    want.extend_from_slice(b"ethereum-rpc.publicnode.com");
    want.extend_from_slice(&[1, 187]);
    assert_eq!(ask, want);
    /* An address literal is a name too: nothing is resolved here. */
    assert_eq!(connect_request("1.2.3.4", 80).map(|a| a[3]), Some(3));
}

#[test]
fn a_name_socks_cannot_carry_is_refused() {
    assert_eq!(connect_request("", 443), None);
    let longest = "a".repeat(255);
    assert_eq!(connect_request(&longest, 443).map(|a| a.len()), Some(7 + 255));
    assert_eq!(connect_request(&"a".repeat(256), 443), None);
}

#[test]
fn the_method_reply() {
    assert_eq!(method_reply(&[]), Parsed::Need);
    assert_eq!(method_reply(&[5]), Parsed::Need);
    assert_eq!(method_reply(&[5, 0]), Parsed::Done(true, 2));
    assert_eq!(method_reply(&[5, 0xFF]), Parsed::Done(false, 2));
    assert_eq!(method_reply(&[5, 0, 9, 9]), Parsed::Done(true, 2));
    assert_eq!(method_reply(&[4, 0]), Parsed::Bad);
}

#[test]
fn the_connect_reply_is_read_only_once_whole() {
    let v4 = [5, 0, 0, 1, 0, 0, 0, 0, 0, 0];
    for cut in 0..v4.len() {
        assert_eq!(connect_reply(&v4[..cut]), Parsed::Need, "cut at {cut}");
    }
    assert_eq!(connect_reply(&v4), Parsed::Done(0, 10));
    /* Stream bytes behind it are not taken with it. */
    let mut more = v4.to_vec();
    more.extend_from_slice(b"HTTP");
    assert_eq!(connect_reply(&more), Parsed::Done(0, 10));
    let v6 = [&[5u8, 4, 0, 4][..], &[0; 18]].concat();
    assert_eq!(connect_reply(&v6[..21]), Parsed::Need);
    assert_eq!(connect_reply(&v6), Parsed::Done(4, 22));
    let name = [5, 5, 0, 3, 3, b'a', b'b', b'c', 0, 80];
    assert_eq!(connect_reply(&name[..4]), Parsed::Need);
    assert_eq!(connect_reply(&name[..9]), Parsed::Need);
    assert_eq!(connect_reply(&name), Parsed::Done(5, 10));
}

#[test]
fn a_reply_that_is_not_socks5_is_refused() {
    assert_eq!(connect_reply(&[4, 0, 0, 1, 0, 0, 0, 0, 0, 0]), Parsed::Bad);
    assert_eq!(connect_reply(&[5, 0, 0, 2, 0, 0, 0, 0, 0, 0]), Parsed::Bad);
    assert_eq!(connect_reply(&[5, 0, 0, 0xFF]), Parsed::Bad);
}

#[test]
fn every_refusal_names_its_network() {
    for rep in 0..=u8::MAX {
        assert!(Proxy::Nym.refused(rep).contains("Nym"), "{rep}");
        assert!(Proxy::Anyone.refused(rep).contains("Anyone"), "{rep}");
    }
    assert_eq!(Proxy::Nym.refused(3), "the Nym mixnet is not connected yet");
    assert_eq!(Proxy::Anyone.refused(4), "the Anyone exit could not resolve the host");
}
