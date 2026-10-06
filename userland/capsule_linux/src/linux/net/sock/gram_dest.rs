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

//! Where a datagram goes, and which socket holds an IPv4 port.

use crate::linux::abi::errno::ECONNREFUSED;

use super::table::Socks;
use super::types::{Addr, Proto};

/// Where a datagram goes: where the socket connected, an IPv4 address, or
/// a Unix socket already found by its name (`unix_name::find`).
pub enum Dest {
    Default,
    Inet(Addr),
    Sock(u32),
}

impl Socks {
    /// The socket bound to `to`; None drops the datagram, and a connected
    /// sender is told on its next call.
    pub(super) fn inet_target(&mut self, id: u32, to: Addr, connected: bool) -> Option<u32> {
        let found = self.bound(Proto::Dgram, to);
        if found.is_none() {
            if let Some(s) = self.get_mut(id).filter(|_| connected) {
                s.error = ECONNREFUSED;
            }
        }
        found
    }
}
