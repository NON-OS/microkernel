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

//! The fake socket's side of the fetch machine's calls.

use super::fetch_wire::FakeWire;
use crate::browser::fetch::wire::{HostFail, Refused, Resolved, Wire};
use crate::browser::net::mixnet::{Network, Way};
use nonos_route_link::Proxy;

impl FakeWire {
    /// Everything sent on `handle`, in order.
    pub fn sent_on(&self, handle: u32) -> Vec<u8> {
        self.sent.iter().filter(|(h, _)| *h == handle).flat_map(|(_, b)| b.clone()).collect()
    }
}

impl Wire for FakeWire {
    fn rtc_now(&self) -> u64 {
        20260601000000
    }
    fn wall_ms(&self) -> i64 {
        self.now.get()
    }
    fn way(&self, host: &str) -> Way {
        if let Some(routes) = self.routes {
            return routes.way(host);
        }
        match (self.mixnet, self.anyone) {
            (false, _) => Way::Direct,
            (true, false) => Way::Proxy { net: Network::Nym, port: 41, proxy: Proxy::Nym },
            (true, true) => Way::Proxy { net: Network::Anyone, port: 42, proxy: Proxy::Anyone },
        }
    }
    fn open(&mut self, way: Way) -> Result<u32, Refused> {
        self.ways.push(way);
        self.next += 1;
        self.opened.push(self.next);
        Ok(self.next)
    }
    fn cached(&self, _host: &str) -> Option<[u8; 4]> {
        None
    }
    fn resolve(&mut self, host: &str) -> Resolved {
        self.resolves += 1;
        if let Some(found) = self.resolver {
            return found;
        }
        match self.names.iter().find(|(name, _)| name == host) {
            Some((_, ip)) => Resolved::Ip(*ip),
            None => Resolved::Unknown,
        }
    }
    fn connect_nb(&mut self, handle: u32, ip: [u8; 4], port: u16) -> Result<(), Refused> {
        self.connects.push((handle, ip, port));
        Ok(())
    }
    fn connect_host(&mut self, handle: u32, host: &str, _port: u16) -> Result<(), HostFail> {
        self.waited.push((handle, String::from(host)));
        self.host_fail.map_or(Ok(()), Err)
    }
    fn poll(&mut self, _handle: u32) -> Result<u8, Refused> {
        self.polls += 1;
        Ok(if self.writable { 2 } else { 0 })
    }
    fn send(&mut self, handle: u32, bytes: &[u8]) -> Result<(), Refused> {
        self.sent.push((handle, bytes.to_vec()));
        Ok(())
    }
    fn sending(&mut self, handle: u32) -> Result<bool, Refused> {
        if !self.unanswered.contains(&handle) {
            return Ok(false);
        }
        self.asks += 1;
        if self.refusing {
            return Err(Refused);
        }
        Ok(true)
    }
    fn close(&mut self, handle: u32) {
        self.closed.push(handle);
    }
    fn room(&self, way: Way) -> bool {
        let Way::Proxy { port, .. } = way else { return true };
        let open = self.opened.iter().zip(&self.ways).filter(|(h, w)| {
            matches!(w, Way::Proxy { port: p, .. } if *p == port) && !self.closed.contains(h)
        });
        self.streams.is_none_or(|cap| open.count() < cap)
    }
    fn broke(&self, handle: u32) -> Option<&'static str> {
        self.broken.iter().find(|(h, _)| *h == handle).map(|(_, code)| *code)
    }
    fn trace(&mut self, _line: &[u8]) {}
}
