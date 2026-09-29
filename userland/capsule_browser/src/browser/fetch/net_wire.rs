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

use super::wire::{Resolved, Wire};
use crate::browser::net::{self, Recv, Source};

/// net.sockets, on the service port it answers at.
pub struct NetWire(pub u32);

impl Source for NetWire {
    fn recv(&mut self, handle: u32, out: &mut [u8]) -> Recv {
        net::socket_recv(self.0, handle, out)
    }

    fn now_ms(&self) -> i64 {
        nonos_libc::mk_time_millis()
    }
}

impl Wire for NetWire {
    fn rtc_now(&self) -> u64 {
        nonos_tls::rtc_now()
    }
    fn mixnet(&self) -> bool {
        net::mixnet::is_on()
    }
    fn open(&mut self) -> Result<u32, ()> {
        net::socket_open(self.0)
    }
    fn cached(&self, host: &str) -> Option<[u8; 4]> {
        net::cached(host)
    }
    fn resolve(&mut self, host: &str) -> Resolved {
        net::resolve(host)
    }
    fn connect_nb(&mut self, handle: u32, ip: [u8; 4], port: u16) -> Result<(), ()> {
        net::socket_connect_nb(self.0, handle, ip, port)
    }
    fn connect_host(&mut self, handle: u32, host: &str, port: u16) -> Result<(), ()> {
        net::socket_connect_host(self.0, handle, host, port)
    }
    fn poll(&mut self, handle: u32) -> Result<u8, ()> {
        net::socket_poll(self.0, handle)
    }
    fn send(&mut self, handle: u32, bytes: &[u8]) -> Result<(), ()> {
        net::socket_send(self.0, handle, bytes)
    }
    fn close(&mut self, handle: u32) {
        let _ = net::socket_close(self.0, handle);
    }
    fn trace(&mut self, line: &[u8]) {
        mk_debug(line.as_ptr(), line.len());
    }
}
