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

//! net.core's listener step on a real smoltcp stack: clients connect to a
//! port net.core listens on, the stack is polled as the serve loop polls it
//! and the step runs after each poll, as iface/poll.rs runs it.

use smoltcp::iface::{Config, Interface, SocketHandle, SocketSet};
use smoltcp::socket::tcp::{Socket, SocketBuffer, State};
use smoltcp::time::{Duration, Instant};
use smoltcp::wire::{HardwareAddress, IpAddress, IpCidr, IpEndpoint};

use super::link::Link;
use super::listeners::arm::{arm, ready};
use super::listeners::table::Listeners;
use crate::tcp_liveness;

const PORT: u16 = 2222;
const OWNER: u32 = 31;

fn socket() -> Socket<'static> {
    let mut s = Socket::new(SocketBuffer::new(vec![0; 4096]), SocketBuffer::new(vec![0; 4096]));
    tcp_liveness::arm(&mut s);
    s
}

struct Host<const B: usize> {
    link: Link,
    iface: Interface,
    sockets: SocketSet<'static>,
    table: Listeners<SocketHandle, 1, B>,
    handle: u32,
    now: Instant,
}

impl<const B: usize> Host<B> {
    fn listening() -> Self {
        let mut link = Link::default();
        let mut iface = Interface::new(Config::new(HardwareAddress::Ip), &mut link, Instant::ZERO);
        iface.update_ip_addrs(|a| a.push(IpCidr::new(IpAddress::v4(10, 0, 0, 1), 24)).unwrap());
        let mut sockets = SocketSet::new(vec![]);
        let mut listener = socket();
        listener.listen(PORT).unwrap();
        let listener = sockets.add(listener);
        let mut table = Listeners::new();
        let handle = table.open(OWNER, PORT, listener).unwrap();
        Host { link, iface, sockets, table, handle, now: Instant::ZERO }
    }

    fn client(&mut self, local: u16) -> SocketHandle {
        let mut c = socket();
        c.connect(self.iface.context(), (IpAddress::v4(10, 0, 0, 1), PORT), local).unwrap();
        self.sockets.add(c)
    }

    fn run(&mut self, span: Duration) {
        let end = self.now + span;
        while self.now < end {
            self.now += Duration::from_millis(100);
            self.iface.poll(self.now, &mut self.link, &mut self.sockets);
            let sockets = &mut self.sockets;
            self.table.for_each_mut(|l| {
                for h in arm(l, sockets, socket) {
                    sockets.remove(h);
                }
            });
        }
    }

    fn accept(&mut self) -> Option<SocketHandle> {
        let sockets = &self.sockets;
        self.table.get_mut(self.handle, OWNER)?.take(|h| ready(sockets.get::<Socket>(h).state()))
    }

    fn readable(&mut self) -> bool {
        let sockets = &self.sockets;
        self.table
            .get_mut(self.handle, OWNER)
            .unwrap()
            .any_ready(|h| ready(sockets.get::<Socket>(h).state()))
    }

    fn listener_state(&mut self) -> State {
        let h = self.table.get_mut(self.handle, OWNER).unwrap().socket;
        self.sockets.get::<Socket>(h).state()
    }

    fn state(&self, h: SocketHandle) -> State {
        self.sockets.get::<Socket>(h).state()
    }

    fn remote_port(&self, h: SocketHandle) -> Option<u16> {
        self.sockets.get::<Socket>(h).remote_endpoint().map(|e: IpEndpoint| e.port)
    }
}

#[test]
fn clients_one_after_another_are_accepted_in_order_and_the_port_keeps_listening() {
    let mut host = Host::<4>::listening();
    let a = host.client(40000);
    host.run(Duration::from_millis(500));
    let b = host.client(40001);
    host.run(Duration::from_secs(1));
    assert_eq!(host.state(a), State::Established);
    assert_eq!(host.state(b), State::Established, "the second was not reset");
    assert_eq!(host.listener_state(), State::Listen);
    assert!(host.readable());
    let first = host.accept().unwrap();
    let second = host.accept().unwrap();
    assert!(!host.readable(), "nothing left to accept");
    assert_eq!(host.remote_port(first), Some(40000));
    assert_eq!(host.remote_port(second), Some(40001));
    assert_eq!(host.accept(), None);
}

#[test]
fn bytes_sent_before_the_accept_are_read_from_the_accepted_connection() {
    let mut host = Host::<4>::listening();
    let a = host.client(40000);
    host.run(Duration::from_millis(500));
    host.sockets.get_mut::<Socket>(a).send_slice(b"NSH1").unwrap();
    host.run(Duration::from_millis(500));
    let child = host.accept().unwrap();
    let mut buf = [0u8; 16];
    let n = host.sockets.get_mut::<Socket>(child).recv_slice(&mut buf).unwrap();
    assert_eq!(&buf[..n], b"NSH1");
    host.sockets.get_mut::<Socket>(child).send_slice(b"ok").unwrap();
    host.run(Duration::from_millis(500));
    let n = host.sockets.get_mut::<Socket>(a).recv_slice(&mut buf).unwrap();
    assert_eq!(&buf[..n], b"ok", "the reply reaches the client that sent");
}

#[test]
fn a_full_backlog_holds_one_on_the_listener_resets_the_next_and_frees_on_accept() {
    let mut host = Host::<1>::listening();
    let a = host.client(40000);
    host.run(Duration::from_millis(500));
    let b = host.client(40001);
    host.run(Duration::from_millis(500));
    let c = host.client(40002);
    host.run(Duration::from_secs(1));
    assert_eq!(host.state(a), State::Established);
    assert_eq!(host.state(b), State::Established, "held on the listening socket");
    assert_ne!(host.listener_state(), State::Listen);
    assert_eq!(host.state(c), State::Closed, "no socket listens, so the SYN was reset");
    let first = host.accept().unwrap();
    assert_eq!(host.remote_port(first), Some(40000));
    host.run(Duration::from_millis(200));
    assert_eq!(host.listener_state(), State::Listen, "room made, the port listens again");
    let second = host.accept().unwrap();
    assert_eq!(host.remote_port(second), Some(40001));
}

#[test]
fn a_connection_the_peer_resets_before_accept_leaves_the_queue_and_the_stack() {
    let mut host = Host::<4>::listening();
    let a = host.client(40000);
    host.run(Duration::from_millis(500));
    let before = host.sockets.iter().count();
    host.sockets.get_mut::<Socket>(a).abort();
    host.run(Duration::from_millis(500));
    assert_eq!(host.accept(), None);
    assert_eq!(host.sockets.iter().count(), before - 1, "its socket was removed");
    assert_eq!(host.listener_state(), State::Listen);
}

#[test]
fn closing_the_listener_gives_back_every_socket_it_held() {
    let mut host = Host::<4>::listening();
    host.client(40000);
    host.run(Duration::from_millis(500));
    let held = host.table.close(host.handle, OWNER).unwrap();
    assert_eq!(held.len(), 2, "the fresh listener and the queued connection");
    assert!(held.iter().all(|h| host.sockets.iter().any(|(x, _)| x == *h)));
}

#[test]
fn a_backlog_of_half_open_syns_gives_way_to_a_client_that_finishes() {
    let mut host = Host::<1>::listening();
    host.link.drop_to_port = Some(40001);
    let silent = host.client(40001);
    host.run(Duration::from_millis(500));
    assert_eq!(host.state(silent), State::SynSent, "it never heard the SYN-ACK");
    assert_eq!(host.listener_state(), State::Listen, "its half-open socket was queued");
    assert_eq!(host.accept(), None, "a half-open connection is not handed out");
    let a = host.client(40000);
    host.run(Duration::from_secs(1));
    assert_eq!(host.state(a), State::Established);
    let child = host.accept().expect("the half-open one gave way");
    assert_eq!(host.remote_port(child), Some(40000));
    // The silent peer's SYN is sent again and taken again; it still keeps
    // no client that finishes out.
    host.run(Duration::from_secs(2));
    let b = host.client(40002);
    host.run(Duration::from_secs(1));
    assert_eq!(host.state(b), State::Established);
    let child = host.accept().expect("again the half-open one gave way");
    assert_eq!(host.remote_port(child), Some(40002));
}
