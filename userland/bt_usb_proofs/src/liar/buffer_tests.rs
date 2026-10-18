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

//! A controller that keeps the ACL buffers it was given and never counts
//! them back. The link's PDUs wait whole; once one has waited longer than
//! a link can stay up unanswered (32 s), the link ends, and the buffers it
//! held are free again once its Disconnection Complete comes.

use crate::air::{Act, Lies, HANDLE};
use crate::fixture::{air, keyboard, link};
use crate::link::{hang_up, End};
use nonos_bt_wire::hci::{acl, Boundary};
use nonos_bt_wire::l2cap::basic;
use nonos_bt_wire::l2cap::cid::ATT;

/// A Read Request (Vol 3, Part F, 3.4.4.3) from the peer, which this host's
/// own database answers, so each one asks for a buffer.
fn read() -> Vec<Vec<u8>> {
    let pdu = basic(ATT, &[0x0a, 0x03, 0x00]).unwrap();
    vec![acl(HANDLE, Boundary::FirstFlushable, &pdu).unwrap()]
}

#[test]
fn buffers_never_given_back_end_the_link_and_are_free_after_it() {
    let (mut host, us, mut env) = keyboard();
    let total = host.acl_free();
    let mut script = vec![Act::Lie(|l: &mut Lies| l.withhold = true)];
    script.extend((0..8).map(|_| Act::Acl(read())));
    air(&mut host).script.extend(script);
    let (end, _) = link(&mut host, &us, &mut env).unwrap();
    assert_eq!(end, End::GaveUp("the controller gives no ACL buffers back"));
    assert_eq!(host.acl_free(), 0);
    assert!(env.now >= 32_000, "ended at {} ms", env.now);
    hang_up(&mut host, HANDLE).unwrap();
    assert_eq!(host.acl_free(), total);
}

/// Disconnect answered Unknown Connection Identifier: the controller has no
/// such link, so it holds no buffers for it either.
#[test]
fn a_link_the_controller_no_longer_has_holds_no_buffers() {
    let (mut host, us, mut env) = keyboard();
    let total = host.acl_free();
    let mut script = vec![Act::Lie(|l: &mut Lies| l.withhold = true)];
    script.extend((0..8).map(|_| Act::Acl(read())));
    air(&mut host).script.extend(script);
    link(&mut host, &us, &mut env).unwrap();
    air(&mut host).lies.forgot = true;
    hang_up(&mut host, HANDLE).unwrap();
    assert_eq!(host.acl_free(), total);
}
