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

//! A guest's stream out of the family, from its connect to its close, over
//! the network chosen (guest_route.rs), and the SOCKS endpoint that opens
//! one the same way for a program that speaks SOCKS (endpoint.rs).
//!
//! - On Nym it is a named stream through net.socks5, by name (nym.rs),
//!   opened and carried in slices (link.rs), and stays in this table.
//! - On Anyone it is a stream net.anon opens (anyone.rs); once open the
//!   socket holds it as its own net.anon stream (sock/adopt.rs).
//! - A network not ready yet is asked again each second, up to the 45 s an
//!   install allows, and then refused (retry.rs). A destination that
//!   refuses is refused at once. Nothing falls back to another network.
//! - A stream that did not open is said in the run's window by network and
//!   reason, never by host or port (line.rs, said.rs).

mod adopt;
mod answer;
mod anyone;
mod apply;
mod ask;
mod bits;
mod connect;
mod drive;
mod endpoint;
mod entry;
mod hear;
mod io;
mod line;
mod link;
mod link_io;
mod nym;
mod nym_errno;
mod outcome;
mod phase;
mod pump;
mod retry;
mod route;
mod said;
mod socks;
mod step;
mod table;
mod take;

pub use bits::bits;
pub use connect::{again, dial};
pub use endpoint::{open as socks, serves};
pub use entry::Dest;
pub use io::{close, send, SEND_MAX};
pub use take::take;
