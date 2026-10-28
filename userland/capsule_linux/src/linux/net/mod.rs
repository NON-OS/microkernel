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

//! Sockets: the family's own, kept in `sock`, and a stream outside the
//! family over the network chosen (guest_route.rs), which routed/ opens and
//! carries: through net.socks5 on the Nym mixnet, net.anon on Anyone.

mod accept;
mod anon_answer;
mod anon_call;
mod anon_ops;
mod anon_stream;
mod anon_wire;
mod api;
mod bind;
mod bind_any;
mod call_kind;
mod cap;
mod close;
mod connect;
mod dest;
mod dgram;
mod dgram_addr;
pub mod dns;
mod fd;
mod flags;
mod guest_route;
mod iov;
mod listen;
mod mmsg;
mod msg;
mod name;
mod named;
mod offline;
mod opt;
mod pair;
mod peer_addr;
mod policy;
mod poll;
/* A development build's local mirror, dialled directly (dev-mirror). */
#[cfg(feature = "dev-mirror")]
pub mod raw;
#[cfg(feature = "dev-mirror")]
pub mod raw_io;
mod ready_sets;
mod recvfrom;
mod resolver;
#[cfg(feature = "dev-mirror")]
pub mod route;
mod routed;
mod select;
mod shutdown;
pub mod sock;
mod sockaddr;
mod socket;
mod socket_kind;
#[cfg(feature = "dev-mirror")]
mod sockets;
mod try_call;
mod xfer;

/* The route every capsule holding Network leaves by, and its bounds (routed/). */
use nonos_route_link::{pick, Proxy, Route, ANYONE_DOWN, NOT_YET_GAP_MS, OPEN_MS};

pub use api::*;
pub use offline::{any_inet, refuse_socket};
