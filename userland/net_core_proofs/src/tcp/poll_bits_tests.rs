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

//! POLL's finished bit on a real smoltcp connection: set once the peer's FIN
//! has come and every byte before it was read, or once the connection is
//! reset; never while the peer is only quiet, still connecting, or while
//! bytes wait to be read.

use smoltcp::iface::SocketHandle;
use smoltcp::socket::tcp::{Socket, State};
use smoltcp::time::Duration;

use super::pair::Pair;
use super::poll_bits::poll_bits;
use crate::protocol::tcp::{POLL_FINISHED, POLL_READABLE, POLL_WRITABLE};

fn bits(p: &Pair, h: SocketHandle) -> u8 {
    let s = p.sockets.get::<Socket>(h);
    poll_bits(s.state(), s.can_recv(), s.can_send())
}

fn drain(p: &mut Pair, h: SocketHandle) -> Vec<u8> {
    let mut buf = [0u8; 64];
    let n = p.sockets.get_mut::<Socket>(h).recv_slice(&mut buf).unwrap_or(0);
    buf[..n].to_vec()
}

#[test]
fn a_quiet_peer_is_writable_and_not_finished() {
    let mut p = Pair::established();
    p.run(Duration::from_secs(5));
    assert_eq!(bits(&p, p.server), POLL_WRITABLE);
    assert_eq!(bits(&p, p.client), POLL_WRITABLE);
}

#[test]
fn bytes_waiting_are_readable_and_not_finished() {
    let mut p = Pair::established();
    let client = p.client;
    p.sockets.get_mut::<Socket>(client).send_slice(b"hello").unwrap();
    p.run(Duration::from_secs(1));
    assert_eq!(bits(&p, p.server), POLL_READABLE | POLL_WRITABLE);
}

#[test]
fn a_fin_after_bytes_is_finished_only_once_they_are_read() {
    let mut p = Pair::established();
    let (client, server) = (p.client, p.server);
    p.sockets.get_mut::<Socket>(client).send_slice(b"last words").unwrap();
    p.sockets.get_mut::<Socket>(client).close();
    p.run(Duration::from_secs(1));
    assert_eq!(p.state(server), State::CloseWait);
    assert_eq!(bits(&p, server) & POLL_FINISHED, 0, "bytes before the FIN are still unread");
    assert_ne!(bits(&p, server) & POLL_READABLE, 0);
    assert_eq!(drain(&mut p, server), b"last words");
    assert_eq!(bits(&p, server) & POLL_FINISHED, POLL_FINISHED);
    assert_eq!(bits(&p, server) & POLL_READABLE, 0);
}

#[test]
fn a_reset_connection_is_finished() {
    let mut p = Pair::established();
    let (client, server) = (p.client, p.server);
    p.sockets.get_mut::<Socket>(client).abort();
    p.run(Duration::from_secs(1));
    assert_eq!(p.state(server), State::Closed);
    assert_eq!(bits(&p, server), POLL_FINISHED);
}

#[test]
fn our_own_close_leaves_the_peer_free_to_send() {
    let mut p = Pair::established();
    let server = p.server;
    p.sockets.get_mut::<Socket>(server).close();
    p.run(Duration::from_secs(1));
    assert_eq!(p.state(server), State::FinWait2);
    assert_eq!(bits(&p, server) & POLL_FINISHED, 0);
}

#[test]
fn finished_is_never_set_with_bytes_to_read_or_before_the_handshake() {
    let all = [
        State::Closed,
        State::Listen,
        State::SynSent,
        State::SynReceived,
        State::Established,
        State::FinWait1,
        State::FinWait2,
        State::CloseWait,
        State::Closing,
        State::LastAck,
        State::TimeWait,
    ];
    for state in all {
        for send in [false, true] {
            assert_eq!(poll_bits(state, true, send) & POLL_FINISHED, 0, "{state}");
        }
    }
    for state in [State::Listen, State::SynSent, State::SynReceived, State::Established] {
        assert_eq!(poll_bits(state, false, false) & POLL_FINISHED, 0, "{state}");
    }
    for state in [State::FinWait1, State::FinWait2] {
        assert_eq!(poll_bits(state, false, true) & POLL_FINISHED, 0, "{state}");
    }
}

#[test]
fn the_bits_are_distinct() {
    assert_eq!(POLL_READABLE & POLL_WRITABLE, 0);
    assert_eq!((POLL_READABLE | POLL_WRITABLE) & POLL_FINISHED, 0);
}
