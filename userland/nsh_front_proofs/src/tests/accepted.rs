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

//! The accept loop reopens a listener net.sockets says is gone, to the
//! accept loop or to a session, and only then: nothing waiting and no room
//! both leave it listening.

use crate::accepted::{accepted, Accept, E_NO_TRANSPORT, E_TABLE_FULL};

const SERVER: &str = include_str!("../../../capsule_nsh_front/src/server.rs");

#[test]
fn a_child_is_taken_only_from_a_whole_answer() {
    assert_eq!(accepted(Some((0, Some(9)))), Accept::Child(9));
    assert_eq!(accepted(Some((0, None))), Accept::Gone, "an answer with no handle");
}

#[test]
fn nothing_waiting_and_no_room_keep_the_listener() {
    assert_eq!(accepted(Some((E_NO_TRANSPORT, None))), Accept::Pending);
    assert_eq!(accepted(Some((E_TABLE_FULL, None))), Accept::Pending);
}

#[test]
fn every_other_answer_reopens_it() {
    assert_eq!(accepted(None), Accept::Gone, "net.sockets did not answer");
    for errno in (1..=u16::MAX).filter(|e| ![E_NO_TRANSPORT, E_TABLE_FULL].contains(e)) {
        assert_eq!(accepted(Some((errno, None))), Accept::Gone, "errno {errno}");
    }
    let at = |s: &str| SERVER.find(s).unwrap_or_else(|| panic!("{s} is gone"));
    assert!(at("Accept::Gone => false,") < at("if !listening {"));
    assert!(at("if !listening {") < at("peers.net.close(listener);"));
    let reopen = SERVER.rfind("listener = open(&mut peers.net);").expect("the reopen");
    assert!(at("peers.net.close(listener);") < reopen);
}

#[test]
fn a_session_that_found_the_listener_gone_has_it_reopened() {
    let at = |s: &str| SERVER.find(s).unwrap_or_else(|| panic!("{s} is gone"));
    let served =
        at("Accept::Child(child) => serve(&mut peers.net, child, attest, listener, &mut marks),");
    assert!(served < at("if !listening {"), "what serve says decides the reopen");
}

#[test]
fn the_errnos_are_net_sockets_own() {
    let errno = include_str!("../../../capsule_net_sockets/src/protocol/errno.rs");
    assert!(errno.contains(&format!("pub const E_NO_TRANSPORT: u16 = {E_NO_TRANSPORT};")));
    assert!(errno.contains(&format!("pub const E_TABLE_FULL: u16 = {E_TABLE_FULL};")));
}
