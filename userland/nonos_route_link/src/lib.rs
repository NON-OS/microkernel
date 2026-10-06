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

//! Leaving through the network the person chose.
//!
//! `Route::chosen` reads the system's default network and which anonymity
//! networks run, and `pick` turns that into a route: the Nym mixnet, the
//! Anyone onion network, a direct connection only when that is the default,
//! or no route at all with the reason. `RouteStream` carries bytes over it,
//! and `direct_refusal` says whether a contact that cannot cross an
//! anonymity network (a time server, ICMP, a lookup in the clear) may be
//! made at all.
//!
//! Everything that decides, frames or reads a proxy's bytes is pure and is
//! compiled into route_link_proofs from these files; `ipc`, `chosen` and
//! `stream` are the thin parts that make the calls.

#![no_std]

extern crate alloc;

mod answer;
mod bounds;
mod carrier;
mod chosen;
mod describe;
mod direct_only;
mod frame;
mod ipc;
mod opening;
mod pick;
mod refusal;
mod slice;
mod socks;
mod stream;
mod tunnel;
mod tunnel_io;

pub use chosen::{default_route, direct_refusal};
pub use direct_only::direct_refused;
pub use pick::{
    for_host, install_route, is_anyone, is_short_anyone, pick, private_only, Route, ANYONE_DOWN,
    ANYONE_INSTALLS_DOWN, ANYONE_NEEDED, NYM_DOWN, PRIVATE_DOWN, SHORT_NOTICE, UNREAD,
};
pub use refusal::Proxy;
pub use slice::Slice;
pub use stream::{RouteOpening, RouteStream};
