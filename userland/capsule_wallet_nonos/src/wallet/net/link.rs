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

//! One connection to the RPC host, over the network the person chose.
//!
//! Every balance, nonce, fee, broadcast and receipt goes over one of these.
//! Each used to resolve the host through net.dns in the clear and dial it
//! from this machine's address, so the RPC provider could tie this machine
//! to every address it asked about, whatever network had been chosen. Now
//! Direct alone does that, as it always did: net.dns, then a stream socket
//! through net.sockets. Under the Nym mixnet or the Anyone network the host
//! name goes unresolved to the exit through net.socks5 or net.anon, and a
//! route that is down refuses with its reason. Nothing falls back. A link
//! is opened and carried a step at a time (`step::open`, `step::io`), so the
//! window never waits on one.

use nonos_route_link::RouteStream;

pub enum Link {
    /// Through net.sockets, to the address net.dns gave: Direct only.
    Direct { sockets: u32, handle: u32 },
    /// Through net.socks5 or net.anon, waiting up to `patience_ms` for the
    /// far end's first byte.
    Routed { stream: RouteStream, patience_ms: u64 },
}

/// How far an open got when it failed, for the probe, and why.
pub struct Reach {
    pub resolve: bool,
    pub socket: bool,
    pub why: &'static str,
}

impl Drop for Link {
    /* A routed stream resets its tunnel as it goes. */
    fn drop(&mut self) {
        if let Link::Direct { sockets, handle } = self {
            let _ = super::socket_close::socket_close(*sockets, *handle);
        }
    }
}
