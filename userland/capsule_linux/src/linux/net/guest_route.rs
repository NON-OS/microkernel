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

//! Which network a guest's stream to an address outside the family leaves
//! through. The rule, read for every connection:
//!
//! - The chosen network is the system's default as nonos_route_link reads
//!   it (`Route::chosen`: the policy store's default network and which
//!   anonymity networks run), turned into a route by its own `pick`, so a
//!   guest and every capsule holding Network cannot disagree about it.
//! - Anyone chosen: the stream goes to net.anon's handle front. Anyone
//!   chosen with net.anon not running is ENETUNREACH, said on the [LINUX]
//!   line by name, and nothing else is tried.
//! - Nym chosen, or a default that could not be read: net.sockets' mixnet
//!   socket, as before. A mixnet that is down is net.sockets' own answer (no
//!   transport), which is ENETUNREACH as before.
//! - Direct chosen: the guest stays on the Nym mixnet. A guest never reaches
//!   the network directly, whatever the person chose. That is deliberate:
//!   the personality hosts programs nobody here wrote, and a direct socket
//!   would name this machine to whatever they connect to (connect_out.rs,
//!   design/install-network.md). It is not loosened here.
//!
//! No failure on one network is answered by trying another: `path` names one
//! network or none, and the connect that follows goes there only.

use super::{Route, ANYONE_DOWN};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Path {
    /// net.sockets' mixnet socket: the Nym mixnet.
    Mixnet,
    /// net.anon's handle front, at this port: the Anyone onion network.
    Anyone(u32),
    /// Nowhere, for this reason: the chosen network is not running.
    Unreachable(&'static str),
}

pub fn path(route: Route) -> Path {
    match route {
        Route::Anon(port) => Path::Anyone(port),
        Route::Down(why) if why == ANYONE_DOWN => Path::Unreachable(why),
        /*
         * The mixnet down, or no default and no mixnet: net.sockets finds no
         * transport and says so, as it did before the route was read.
         */
        Route::Nym(_) | Route::Direct | Route::Down(_) => Path::Mixnet,
    }
}
