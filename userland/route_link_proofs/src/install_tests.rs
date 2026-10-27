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

//! The install mark: only an install's stream carries it, in the greeting
//! and nowhere else, and the rest of the conversation is as any stream's.

use crate::fake::FakeProxy;
use crate::install::{greeting, INSTALL_GREETING, METHOD_INSTALL};
use crate::refusal::Proxy;
use crate::socks::GREETING;
use crate::tunnel::Tunnel;

#[test]
fn the_mark_is_a_private_method_after_no_authentication() {
    assert!((0x80..=0xFE).contains(&METHOD_INSTALL), "RFC 1928's private range");
    assert_eq!(INSTALL_GREETING, [5, 2, 0, METHOD_INSTALL]);
    assert_eq!(greeting(true), INSTALL_GREETING);
    assert_eq!(greeting(false), GREETING, "every other stream greets as before");
}

#[test]
fn an_install_greets_with_the_mark_and_connects_as_any_stream() {
    let opened = Tunnel::open_as(FakeProxy::anon(2), Proxy::Anyone, "example.com", 443, true);
    let Ok(t) = opened else { panic!("did not open") };
    let n = t.carrier.numbered();
    assert_eq!(n[0], (1, INSTALL_GREETING.to_vec()));
    assert_eq!(&n[1].1[..5], &[5, 1, 0, 3, 11], "the CONNECT carries no mark");
    assert_eq!(t.carrier.host, b"example.com");
}

#[test]
fn any_other_stream_greets_without_it() {
    let Ok(t) = Tunnel::open(FakeProxy::anon(2), Proxy::Anyone, "example.com", 443) else {
        panic!("did not open")
    };
    assert_eq!(t.carrier.numbered()[0], (1, GREETING.to_vec()));
    assert!(!t.install);
}
