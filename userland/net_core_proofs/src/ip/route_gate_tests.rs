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

//! ICMP leaves in the clear only on the Direct route: the rule over every
//! value the policy store can give, and its place in SEND_PACKET, read from
//! the source.

use nonos_policy_proto::route;

use super::route_gate::clear_send_allowed;

const SEND: &str = include_str!("../../../capsule_net_core/src/server/handlers/ip/send.rs");

#[test]
fn only_the_direct_route_sends_icmp_in_the_clear() {
    assert!(clear_send_allowed(Some(route::DIRECT)));
    assert!(!clear_send_allowed(None), "a store that does not answer is not Direct");
    for v in (0..=u8::MAX).filter(|v| *v != route::DIRECT) {
        assert!(!clear_send_allowed(Some(v)), "route {v}");
    }
}

#[test]
fn send_packet_asks_the_gate_before_the_socket_is_touched() {
    let gate = SEND.find("if !super::route_gate::clear_send_allowed(route()) {");
    let refusal = SEND.find("return fail(sender_pid, req, E_NOT_DIRECT, tx);");
    let send = SEND.find("with_icmp(ident,");
    assert!(matches!((gate, refusal, send), (Some(g), Some(r), Some(s)) if g < r && r < s));
}

#[test]
fn not_direct_is_an_errno_of_its_own() {
    use crate::protocol::ip::*;
    let others = [E_OK, E_BAD_LEN, E_NO_CONFIG, E_NO_ROUTE, E_NO_NEIGHBOUR, E_EMPTY, E_BAD_PROTO];
    assert!(!others.contains(&E_NOT_DIRECT));
}
