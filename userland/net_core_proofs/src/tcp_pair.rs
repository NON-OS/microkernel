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

//! One TCP connection over `Link`, both ends armed as net.core arms its
//! sockets, with a clock the proofs move by hand.

use smoltcp::iface::{Config, Interface, SocketHandle, SocketSet};
use smoltcp::socket::tcp::{Socket, SocketBuffer, State};
use smoltcp::time::{Duration, Instant};
use smoltcp::wire::{HardwareAddress, IpAddress, IpCidr};

use crate::tcp_link::Link;
use crate::tcp_liveness::arm;

pub struct Pair {
    pub link: Link,
    iface: Interface,
    pub sockets: SocketSet<'static>,
    pub client: SocketHandle,
    pub server: SocketHandle,
    now: Instant,
}

fn socket() -> Socket<'static> {
    let mut s = Socket::new(SocketBuffer::new(vec![0; 4096]), SocketBuffer::new(vec![0; 4096]));
    arm(&mut s);
    s
}

impl Pair {
    /// 10.0.0.1:40000 to 10.0.0.1:80, established.
    pub fn established() -> Pair {
        let mut link = Link::default();
        let mut iface = Interface::new(Config::new(HardwareAddress::Ip), &mut link, Instant::ZERO);
        let addr = IpAddress::v4(10, 0, 0, 1);
        iface.update_ip_addrs(|a| a.push(IpCidr::new(addr, 24)).unwrap());
        let mut sockets = SocketSet::new(vec![]);
        let mut server = socket();
        server.listen(80).unwrap();
        let server = sockets.add(server);
        let mut client = socket();
        client.connect(iface.context(), (addr, 80), 40000).unwrap();
        let client = sockets.add(client);
        let mut p = Pair { link, iface, sockets, client, server, now: Instant::ZERO };
        p.run(Duration::from_secs(1));
        assert_eq!(p.state(client), State::Established);
        p
    }

    /// Poll every 100 ms for `span`, as the serve loop polls an idle stack.
    pub fn run(&mut self, span: Duration) {
        let end = self.now + span;
        while self.now < end {
            self.now += Duration::from_millis(100);
            self.iface.poll(self.now, &mut self.link, &mut self.sockets);
        }
    }

    pub fn state(&self, h: SocketHandle) -> State {
        self.sockets.get::<Socket>(h).state()
    }
}
