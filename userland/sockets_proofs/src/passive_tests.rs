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

//! The passive open as net.sockets holds it (handlers/passive.rs,
//! listen_route.rs): a listen only on a bound stream socket with no
//! transport, only on Direct; an accept only from a socket this app put into
//! listening, so no handle of another kind can name another app's listener.

use crate::protocol::{E_ALREADY_BOUND, E_NOT_BOUND, E_NO_TRANSPORT, E_TABLE_FULL};
use crate::server::handlers::listen_route::listen_route;
use crate::server::handlers::listen_who::listener_allowed;
use crate::server::handlers::passive::{
    accept_parent, accept_refused, listen_port, TCP_NO_ROOM, TCP_RX_EMPTY,
};
use crate::sockets::{Kind, LocalAddr4, Socket, SocketKey};

const LISTEN: &str = include_str!("../../capsule_net_sockets/src/server/handlers/listen.rs");
const ACCEPT: &str = include_str!("../../capsule_net_sockets/src/server/handlers/accept.rs");

/// A listener handle as net.core's net.tcp gives one.
const TCP_LISTENER: u32 = 0x4000_0001;

fn socket(kind: Kind) -> Socket {
    Socket::new(SocketKey { pid: 30, handle: 7 }, kind)
}

fn bound(kind: Kind) -> Socket {
    let mut s = socket(kind);
    s.local = Some(LocalAddr4 { port: 2222 });
    s.bound = true;
    s
}

#[test]
fn a_listen_off_the_direct_route_is_refused() {
    assert_eq!(listen_route(|| true), Ok(()));
    assert_eq!(listen_route(|| false), Err(E_NO_TRANSPORT));
    let at = |s: &str| LISTEN.find(s).unwrap_or_else(|| panic!("{s} is gone"));
    assert!(at("listen_route(direct_chosen)") < at("tcp::listen(state::tcp(), port)"));
}

#[test]
fn only_nsh_front_may_listen() {
    const FRONT: u32 = 40;
    assert!(listener_allowed(FRONT, Some(FRONT)));
    for sender in [0, 1, FRONT - 1, FRONT + 1, u32::MAX] {
        assert!(!listener_allowed(sender, Some(FRONT)), "pid {sender}");
    }
    assert!(!listener_allowed(FRONT, None), "nsh.front not registered");
    assert!(!listener_allowed(0, Some(0)), "pid 0 is no registration");
    let at = |s: &str| LISTEN.find(s).unwrap_or_else(|| panic!("{s} is gone"));
    assert!(at("listener_allowed(pid, nsh_front_pid())") < at("listen_route(direct_chosen)"));
    assert!(at("listen_port(&sock)") < at("listener_allowed(pid, nsh_front_pid())"));
    assert!(LISTEN.contains("b\"nsh.front\""));
}

#[test]
fn only_a_bound_stream_socket_with_no_transport_listens() {
    assert_eq!(listen_port(&bound(Kind::Stream)), Ok(2222));
    assert_eq!(listen_port(&socket(Kind::Stream)), Err(E_NOT_BOUND), "not bound");
    for kind in [Kind::Datagram, Kind::Mixnet] {
        assert_eq!(listen_port(&bound(kind)), Err(E_NOT_BOUND));
    }
    let mut listening = bound(Kind::Stream);
    listening.transport_handle = TCP_LISTENER;
    listening.listening = true;
    assert_eq!(listen_port(&listening), Err(E_ALREADY_BOUND), "a second listener");
    let mut connected = bound(Kind::Stream);
    connected.transport_handle = 3;
    assert_eq!(listen_port(&connected), Err(E_ALREADY_BOUND), "a connection would leak");
    let at = |s: &str| LISTEN.find(s).unwrap_or_else(|| panic!("{s} is gone"));
    assert!(at("listen_port(&sock)") < at("listen_route(direct_chosen)"));
}

#[test]
fn an_accept_takes_only_from_a_socket_this_app_put_into_listening() {
    let mut listening = bound(Kind::Stream);
    listening.transport_handle = TCP_LISTENER;
    listening.listening = true;
    assert_eq!(accept_parent(&listening), Ok(TCP_LISTENER));
    let mut mixnet = socket(Kind::Mixnet);
    mixnet.transport_handle = TCP_LISTENER;
    assert_eq!(accept_parent(&mixnet), Err(E_NOT_BOUND), "a net.nym handle of the same number");
    let mut connected = bound(Kind::Stream);
    connected.transport_handle = TCP_LISTENER;
    assert_eq!(accept_parent(&connected), Err(E_NOT_BOUND), "a stream that is not listening");
    let mut flag_only = bound(Kind::Stream);
    flag_only.listening = true;
    assert_eq!(accept_parent(&flag_only), Err(E_NOT_BOUND), "no listener behind it");
    let mut datagram = bound(Kind::Datagram);
    datagram.listening = true;
    datagram.transport_handle = TCP_LISTENER;
    assert_eq!(accept_parent(&datagram), Err(E_NOT_BOUND));
    let at = |s: &str| ACCEPT.find(s).unwrap_or_else(|| panic!("{s} is gone"));
    assert!(at("accept_parent(&sock)") < at("tcp::accept(state::tcp(), parent)"));
}

#[test]
fn a_listener_net_tcp_no_longer_holds_is_told_gone_and_given_up() {
    assert_eq!(accept_refused(TCP_RX_EMPTY), (E_NO_TRANSPORT, false), "nothing waiting yet");
    assert_eq!(accept_refused(TCP_NO_ROOM), (E_TABLE_FULL, false), "it stays queued");
    for tcp_errno in (0..=u16::MAX).filter(|e| ![TCP_RX_EMPTY, TCP_NO_ROOM].contains(e)) {
        assert_eq!(accept_refused(tcp_errno), (E_NOT_BOUND, true), "net.tcp errno {tcp_errno}");
    }
    let at = |s: &str| ACCEPT.find(s).unwrap_or_else(|| panic!("{s} is gone"));
    let gone = at("if gone {");
    assert!(gone < at("let _ = tcp::close(state::tcp(), parent);"));
    assert!(at("let _ = tcp::close(state::tcp(), parent);") < at("s.listening = false;"));
    assert!(at("s.listening = false;") < at("s.transport_handle = 0;"));
}

#[test]
fn net_core_answers_no_room_and_nothing_waiting_as_net_sockets_reads_them() {
    let tcp = include_str!("../../capsule_net_core/src/protocol/tcp.rs");
    assert!(tcp.contains(&format!("pub const E_RX_EMPTY: u16 = {TCP_RX_EMPTY};")));
    assert!(tcp.contains(&format!("pub const E_NO_ROOM: u16 = {TCP_NO_ROOM};")));
    let core_accept = include_str!("../../capsule_net_core/src/server/handlers/tcp/accept.rs");
    assert!(core_accept.contains("Some(Some(Err(()))) => Err(E_NO_ROOM),"));
    assert!(core_accept.contains("Some(None) => Err(E_RX_EMPTY),"));
}
