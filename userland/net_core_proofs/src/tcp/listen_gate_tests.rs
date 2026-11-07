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

use super::listeners::gate::{accept_allowed, listen_allowed, listeners_may_stay};
use nonos_policy_proto::route;

const SOCKETS: u32 = 31;

#[test]
fn net_sockets_on_direct_may_listen() {
    assert!(listen_allowed(SOCKETS, Some(SOCKETS), Some(route::DIRECT)));
}

#[test]
fn any_other_sender_is_refused_whatever_the_route() {
    for sender in [0, 1, SOCKETS - 1, SOCKETS + 1, u32::MAX] {
        assert!(!listen_allowed(sender, Some(SOCKETS), Some(route::DIRECT)), "pid {sender}");
    }
}

#[test]
fn no_net_sockets_registered_refuses_everyone() {
    assert!(!listen_allowed(SOCKETS, None, Some(route::DIRECT)));
    assert!(!listen_allowed(0, Some(0), Some(route::DIRECT)), "pid 0 is no registration");
}

#[test]
fn every_route_but_direct_and_an_unread_route_refuse() {
    for chosen in [None, Some(route::NYM), Some(route::ANYONE), Some(3), Some(u8::MAX)] {
        assert!(!listen_allowed(SOCKETS, Some(SOCKETS), chosen), "{chosen:?}");
        assert!(!listeners_may_stay(chosen), "{chosen:?}");
    }
    assert!(listeners_may_stay(Some(route::DIRECT)));
}

#[test]
fn an_accept_hands_out_a_connection_only_while_direct_is_chosen() {
    assert!(accept_allowed(Some(route::DIRECT)));
    for chosen in [None, Some(route::NYM), Some(route::ANYONE), Some(3), Some(u8::MAX)] {
        assert!(!accept_allowed(chosen), "{chosen:?}");
    }
}
