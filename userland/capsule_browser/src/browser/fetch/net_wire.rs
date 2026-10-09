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

//! The fetch machine's calls, carried out by net.sockets.

use nonos_libc::mk_debug;

use super::wire::{HostFail, Refused, Resolved, Wire};
use crate::browser::net::mixnet::Way;
use crate::browser::net::{self, Recv, Source};

/// net.sockets, on the service port it answers at.
pub struct NetWire(pub u32);

impl Source for NetWire {
    fn recv(&mut self, handle: u32, out: &mut [u8]) -> Recv {
        net::socket_recv(self.0, handle, out)
    }

    fn now_ms(&self) -> i64 {
        nonos_libc::mk_uptime_ms()
    }
}

impl Wire for NetWire {
    fn rtc_now(&self) -> u64 {
        nonos_tls::rtc_now()
    }
    fn wall_ms(&self) -> i64 {
        nonos_libc::mk_time_millis()
    }
    fn way(&self, host: &str) -> Way {
        net::mixnet::way(host)
    }
    fn open(&mut self, way: Way) -> Result<u32, Refused> {
        net::socket_open(self.0, way).map_err(|()| Refused)
    }
    fn cached(&self, host: &str) -> Option<[u8; 4]> {
        net::cached(host)
    }
    fn resolve(&mut self, host: &str) -> Resolved {
        net::resolve(host)
    }
    fn connect_nb(&mut self, handle: u32, ip: [u8; 4], port: u16) -> Result<(), Refused> {
        net::socket_connect_nb(self.0, handle, ip, port).map_err(|()| Refused)
    }
    fn connect_host(&mut self, handle: u32, host: &str, port: u16) -> Result<(), HostFail> {
        net::socket_connect_host(self.0, handle, host, port).map_err(|status| match status {
            Some(net::E_NO_DNS) => HostFail::NoDns,
            _ => HostFail::Refused,
        })
    }
    fn poll(&mut self, handle: u32) -> Result<u8, Refused> {
        net::socket_poll(self.0, handle).map_err(|()| Refused)
    }
    fn send(&mut self, handle: u32, bytes: &[u8]) -> Result<(), Refused> {
        net::socket_send(self.0, handle, bytes).map_err(|()| Refused)
    }
    fn sending(&mut self, handle: u32) -> Result<bool, Refused> {
        net::socket_sending(handle).map_err(|()| Refused)
    }
    fn close(&mut self, handle: u32) {
        let _ = net::socket_close(self.0, handle);
    }
    fn room(&self, way: Way) -> bool {
        match way {
            Way::Proxy { port, .. } => net::mixnet::room(port),
            Way::Direct | Way::Refused(_) => true,
        }
    }
    fn broke(&self, handle: u32) -> Option<&'static str> {
        net::mixnet::broke(handle).map(super::proxy_fault::code)
    }
    fn trace(&mut self, line: &[u8]) {
        mk_debug(line.as_ptr(), line.len());
    }
}
