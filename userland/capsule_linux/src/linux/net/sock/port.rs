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

//! Ports on the loopback address: who holds one, and a free one when a
//! program asks for port 0 or connects before it binds.

use super::table::Socks;
use super::types::{Addr, Proto};

/// Linux's net.ipv4.ip_local_port_range default.
const FIRST: u16 = 32768;
const LAST: u16 = 60999;

impl Socks {
    /// The socket of this kind bound to exactly this address, if any.
    pub fn bound(&self, proto: Proto, at: Addr) -> Option<u32> {
        self.iter().find(|(_, s)| s.proto == proto && s.local == Some(at)).map(|(i, _)| i)
    }

    /// The listener a connect to `at` from port `from` reaches. Listeners
    /// that share a port with SO_REUSEPORT take connections by the
    /// connecting port, as Linux spreads them by a hash of the connection.
    pub fn listener(&self, at: Addr, from: u16) -> Option<u32> {
        let group: alloc::vec::Vec<u32> = self
            .iter()
            .filter(|(_, s)| s.listening && s.proto == Proto::Stream && s.local == Some(at))
            .map(|(i, _)| i)
            .collect();
        group.get(usize::from(from) % group.len().max(1)).copied()
    }

    /// True when binding `id` to `at` takes a port another socket holds.
    /// Two sockets share one when both set SO_REUSEPORT, or when both set
    /// SO_REUSEADDR and the other does not listen, as Linux allows.
    pub fn in_use(&self, id: u32, at: Addr) -> bool {
        let Some(me) = self.get(id) else {
            return true;
        };
        self.iter().any(|(i, s)| {
            i != id
                && s.proto == me.proto
                && s.local == Some(at)
                && !(s.opts.reuseport && me.opts.reuseport)
                && (s.listening || !(s.opts.reuseaddr && me.opts.reuseaddr))
        })
    }

    /// A port in the ephemeral range no socket of this kind holds on `ip`.
    pub fn ephemeral(&mut self, proto: Proto, ip: [u8; 4]) -> Option<u16> {
        let span = (LAST - FIRST + 1) as u32;
        for step in 0..span {
            let port = FIRST + ((u32::from(self.next_port) + step) % span) as u16;
            if self.bound(proto, Addr { ip, port }).is_none() {
                self.next_port = (port - FIRST + 1) % span as u16;
                return Some(port);
            }
        }
        None
    }
}
