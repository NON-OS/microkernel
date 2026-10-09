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

//! Which network an app's connection leaves through, decided from the
//! system's chosen network as nonos_route_link reads it for every capsule
//! that holds Network, so an app and the system cannot disagree about it.
//!
//! - Direct: a stream socket straight to the host, and a name looked up by
//!   net.dns. Only here does a name leave this machine as a lookup, does a
//!   datagram leave, or does a port take connections from outside.
//! - Nym: net.sockets' mixnet socket, which carries an IPv4 address and a
//!   port only. A connect by name is refused by net.sockets (E_NAME_REFUSED)
//!   rather than resolved in the clear, so on Nym only an address connects.
//! - Anyone: the shared route client's tunnel through net.anon, with the name
//!   unresolved too.
//! - A chosen network that is not running: nothing, and the reason.
//!
//! Before, every connection took a direct socket and every name a lookup in
//! the clear, whatever was chosen. No failure here is answered by trying
//! another network.

use super::Route;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Way {
    Direct,
    Mixnet,
    Anyone(Route),
    Down(&'static str),
}

pub const LOOKUP_OFF_DIRECT: &str =
    "a name is looked up only on the Direct network; on Nym or Anyone the exit resolves it";
pub const DATAGRAMS_OFF_DIRECT: &str = "datagrams leave only on the Direct network";
pub const LISTEN_OFF_DIRECT: &str =
    "only the Direct network takes connections from outside this machine";

/// The way a stream to `route`'s network leaves.
pub fn way(route: Route) -> Way {
    match route {
        Route::Direct => Way::Direct,
        Route::Nym(_) => Way::Mixnet,
        Route::Anon(_) => Way::Anyone(route),
        Route::Down(why) => Way::Down(why),
    }
}

/// Ok when `route` lets a lookup, a datagram or a listener leave this machine
/// as itself, else the reason, `off` when another network is chosen.
pub fn direct_only(route: Route, off: &'static str) -> Result<(), &'static str> {
    match way(route) {
        Way::Direct => Ok(()),
        Way::Down(why) => Err(why),
        Way::Mixnet | Way::Anyone(_) => Err(off),
    }
}
