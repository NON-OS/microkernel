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
//! through, read for every connection from the policy store's default
//! network and which anonymity networks run, by nonos_route_link's own
//! `pick`, so a guest and every capsule holding Network cannot disagree:
//!
//! - Anyone: net.anon's handle front, the host by name.
//! - Nym: net.socks5, a SOCKS5 CONNECT naming the host unresolved, as an
//!   install's own stream goes (routed/nym.rs). Never net.sockets' mixnet
//!   socket, whose frames carry an address and so refuse every name.
//! - Direct: Nym as well. A guest runs a program nobody here wrote, and a
//!   direct socket would name this machine to whatever it reaches
//!   (design/install-network.md). That is deliberate.
//! - A default that cannot be read, or a network whose service has not
//!   registered yet: not yet. The open asks again until its time is up and
//!   is then refused (routed/retry.rs). An unread default never becomes Nym.
//!
//! No failure on one network is answered by trying another.

use super::{pick, Route, ANYONE_DOWN};

/// The network a stream opens on, or waits for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Net {
    Anyone,
    Nym,
    /// The default could not be read, so no network is known.
    Unread,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Path {
    /// net.socks5's SOCKS front, at this port: the Nym mixnet, by name.
    Nym(u32),
    /// net.anon's handle front, at this port: the Anyone onion network.
    Anyone(u32),
    /// Not yet, on this network, for this reason.
    Wait(Net, &'static str),
}

pub const UNREAD: &str = "the default network could not be read from the policy store";
pub const SOCKS_DOWN: &str = "net.socks5, which carries a program's streams, is not running";

/// The path for `default`, the policy store's answer (None when it did not
/// give one), with `nym` and `anon` the ports of net.socks5 and net.anon, 0
/// for a service that has not registered.
pub fn path(default: Option<u8>, nym: u32, anon: u32) -> Path {
    let Some(chosen) = default else {
        return Path::Wait(Net::Unread, UNREAD);
    };
    match pick(Some(chosen), nym, anon) {
        Route::Anon(port) => Path::Anyone(port),
        Route::Nym(port) => Path::Nym(port),
        Route::Direct if nym != 0 => Path::Nym(nym),
        Route::Direct => Path::Wait(Net::Nym, SOCKS_DOWN),
        Route::Down(why) if why == ANYONE_DOWN => Path::Wait(Net::Anyone, why),
        Route::Down(why) => Path::Wait(Net::Nym, why),
    }
}
