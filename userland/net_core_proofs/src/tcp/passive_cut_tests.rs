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

//! A connection a listener took lives only while the Direct route is chosen:
//! once another route is, the reap resets it, so a session accepted on
//! Direct does not go on in the clear. Its handle stays with its owner until
//! that client's CLOSE, so it never comes to name another connection.

use nonos_policy_proto::route;
use smoltcp::iface::SocketHandle;
use smoltcp::socket::tcp::{Socket, State};
use smoltcp::time::Duration;

use super::handles_table::Handles;
use super::listeners::gate::passive_may_stay;
use super::pair::Pair;
use super::poll_bits::poll_bits;
use super::reset::reset;
use crate::protocol::tcp::POLL_FINISHED;

const SOCKETS: u32 = 31;
const REAP: &str = include_str!("../../../capsule_net_core/src/server/reap.rs");
const ACCEPT: &str = include_str!("../../../capsule_net_core/src/server/handlers/tcp/accept.rs");
const SEND: &str = include_str!("../../../capsule_net_core/src/server/handlers/tcp/send.rs");
const RECV: &str = include_str!("../../../capsule_net_core/src/server/handlers/tcp/recv.rs");

#[test]
fn only_accepted_connections_are_passive() {
    let mut t: Handles<u8, 8> = Handles::new();
    let own = t.alloc(SOCKETS, 1).unwrap();
    let took = t.alloc_passive(SOCKETS, 2).unwrap();
    t.alloc(SOCKETS + 1, 3).unwrap();
    assert_eq!(t.passive(), vec![2]);
    assert_eq!(t.get(own, SOCKETS), Some(1));
    assert_eq!(t.get(took, SOCKETS), Some(2));
}

#[test]
fn listing_them_leaves_every_slot_with_its_owner() {
    let mut t: Handles<u8, 8> = Handles::new();
    let took = t.alloc_passive(SOCKETS, 2).unwrap();
    let _ = t.passive();
    assert_eq!(t.get(took, SOCKETS), Some(2));
    assert_eq!(t.alloc(SOCKETS, 9), Some(took + 1), "the slot is not given out again");
}

#[test]
fn a_freed_or_reaped_slot_is_not_passive_when_reused() {
    let mut t: Handles<u8, 8> = Handles::new();
    let took = t.alloc_passive(SOCKETS, 2).unwrap();
    t.free(took, SOCKETS);
    assert_eq!(t.alloc(SOCKETS, 5), Some(took));
    assert!(t.passive().is_empty());

    let took = t.alloc_passive(SOCKETS + 1, 6).unwrap();
    assert_eq!(t.take_ended(|pid| pid != SOCKETS + 1), vec![6]);
    assert_eq!(t.alloc(SOCKETS, 7), Some(took));
    assert!(t.passive().is_empty());

    t.alloc_passive(SOCKETS, 8).unwrap();
    t.forget_all();
    t.alloc(SOCKETS, 9).unwrap();
    assert!(t.passive().is_empty());
}

#[test]
fn a_free_by_someone_else_leaves_it_passive() {
    let mut t: Handles<u8, 8> = Handles::new();
    let took = t.alloc_passive(SOCKETS, 2).unwrap();
    t.free(took, SOCKETS + 1);
    assert_eq!(t.passive(), vec![2]);
}

#[test]
fn passive_connections_share_the_per_owner_bound() {
    let mut t: Handles<u8, 8> = Handles::new();
    for h in 0..Handles::<u8, 8>::PER_OWNER as u8 {
        assert!(t.alloc_passive(SOCKETS, h).is_some());
    }
    assert_eq!(t.alloc_passive(SOCKETS, 99), None);
    assert_eq!(t.alloc(SOCKETS, 99), None);
}

#[test]
fn they_stay_only_on_direct() {
    assert!(passive_may_stay(Some(route::DIRECT)));
    assert!(!passive_may_stay(None), "no answer from the policy store");
    for chosen in (0..=u8::MAX).filter(|r| *r != route::DIRECT) {
        assert!(!passive_may_stay(Some(chosen)), "route {chosen}");
    }
}

#[test]
fn a_cut_connection_resets_its_peer_and_reads_finished() {
    let mut p = Pair::established();
    let (client, server) = (p.client, p.server);
    p.sockets.get_mut::<Socket>(server).abort();
    p.run(Duration::from_secs(1));
    assert_eq!(p.state(client), State::Closed, "the peer was reset");
    let s = p.sockets.get::<Socket>(server);
    assert_eq!(poll_bits(s.state(), s.can_recv(), s.can_send()), POLL_FINISHED);
}

#[test]
fn accept_hands_out_passive_handles_and_the_reap_cuts_them_off_direct() {
    assert!(ACCEPT.contains("handles::alloc_passive(sender_pid, h)"));
    assert!(!ACCEPT.contains("handles::alloc(sender_pid"));
    let at = |s: &str| REAP.find(s).unwrap_or_else(|| panic!("{s} is gone"));
    assert!(at("let passive = handles::passive();") < at("!gate::passive_may_stay(route())"));
    assert!(at("!gate::passive_may_stay(route())") < at("cut = passive;"));
    assert!(at("cut_off(sockets, sock_handle);") < at("fn cut_off("));
    assert!(REAP.contains("reset(sockets.get_mut::<tcp::Socket>(sock_handle));"));
    assert!(!REAP.contains("handles::free"), "a cut connection keeps its slot");
}

#[test]
fn only_its_owner_sees_a_connection_as_passive() {
    let mut t: Handles<u8, 8> = Handles::new();
    let own = t.alloc(SOCKETS, 1).unwrap();
    let took = t.alloc_passive(SOCKETS, 2).unwrap();
    assert!(t.is_passive(took, SOCKETS));
    assert!(!t.is_passive(own, SOCKETS), "a client's own connect");
    assert!(!t.is_passive(took, SOCKETS + 1), "another client's handle");
    assert!(!t.is_passive(0, SOCKETS), "the null handle");
    assert!(!t.is_passive(9, SOCKETS), "past the table");
    t.free(took, SOCKETS);
    assert!(!t.is_passive(took, SOCKETS), "freed");
}

#[test]
fn a_send_on_an_accepted_connection_off_direct_resets_it_and_sends_nothing() {
    let at = |s: &str| SEND.find(s).unwrap_or_else(|| panic!("{s} is gone"));
    let cut = "let cut = handles::is_passive(app_handle, sender_pid) && !gate::passive_may_stay(route());";
    assert!(at(cut) < at("if cut {"));
    assert!(at("if cut {") < at("reset(sock);"));
    assert!(at("reset(sock);") < at("return Err(tcp::SendError::InvalidState);"));
    assert!(at("return Err(tcp::SendError::InvalidState);") < at("sock.send_slice(payload)"));
}

#[test]
fn a_reset_connection_takes_no_more_bytes() {
    let mut p = Pair::established();
    let server = p.server;
    p.sockets.get_mut::<Socket>(server).abort();
    assert!(p.sockets.get_mut::<Socket>(server).send_slice(b"after").is_err());
}

#[test]
fn a_recv_on_an_accepted_connection_off_direct_resets_it_and_reads_nothing() {
    let at = |s: &str| RECV.find(s).unwrap_or_else(|| panic!("{s} is gone"));
    let cut = "let cut = handles::is_passive(app_handle, sender_pid) && !gate::passive_may_stay(route());";
    assert!(at("handles::get(app_handle, sender_pid)") < at(cut));
    assert!(at(cut) < at("if cut {"));
    assert!(at("if cut {") < at("reset(sock);"));
    assert!(at("reset(sock);") < at("return Err(tcp::RecvError::InvalidState);"));
    assert!(at("return Err(tcp::RecvError::InvalidState);") < at("sock.recv_slice(&mut buf)"));
    assert!(!RECV.contains("handles::free"), "a reset connection keeps its slot");
}

#[test]
fn an_abort_alone_still_hands_out_what_the_peer_sent() {
    let mut p = Pair::established();
    let server = queued(&mut p);
    p.sockets.get_mut::<Socket>(server).abort();
    let mut buf = [0u8; 64];
    assert!(p.sockets.get_mut::<Socket>(server).recv_slice(&mut buf).is_ok_and(|n| n > 0));
}

#[test]
fn a_reset_connection_gives_no_more_of_what_its_peer_sent() {
    let mut p = Pair::established();
    let server = queued(&mut p);
    reset(p.sockets.get_mut::<Socket>(server));
    let s = p.sockets.get_mut::<Socket>(server);
    assert!(!s.can_recv(), "the queue is gone");
    let mut buf = [0u8; 64];
    assert!(!s.recv_slice(&mut buf).is_ok_and(|n| n > 0));
    assert_eq!(poll_bits(s.state(), s.can_recv(), s.can_send()), POLL_FINISHED);
    p.run(Duration::from_secs(1));
    assert_eq!(p.state(p.client), State::Closed, "the peer was reset");
}

#[test]
fn a_reset_empties_a_queue_that_wrapped_its_ring() {
    let mut p = Pair::established();
    let (client, server) = (p.client, p.server);
    for _ in 0..3 {
        while p.sockets.get_mut::<Socket>(client).send_slice(&[7u8; 1500]).is_ok_and(|n| n > 0) {
            p.run(Duration::from_millis(50));
        }
        let mut half = [0u8; 3000];
        let _ = p.sockets.get_mut::<Socket>(server).recv_slice(&mut half);
        p.run(Duration::from_millis(200));
    }
    assert!(p.sockets.get::<Socket>(server).can_recv());
    reset(p.sockets.get_mut::<Socket>(server));
    assert!(!p.sockets.get::<Socket>(server).can_recv());
}

/// The server end of `p`, with bytes its peer sent waiting to be read.
fn queued(p: &mut Pair) -> SocketHandle {
    let (client, server) = (p.client, p.server);
    p.sockets.get_mut::<Socket>(client).send_slice(b"queued before the route changed").unwrap();
    p.run(Duration::from_secs(1));
    assert!(p.sockets.get::<Socket>(server).can_recv(), "the bytes arrived");
    server
}
