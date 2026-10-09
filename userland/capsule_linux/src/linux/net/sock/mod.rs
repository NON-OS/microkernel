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

//! The family's sockets: one table this personality keeps for every process
//! it hosts, so a connection between two of them, or two threads of one,
//! never leaves the capsule. A descriptor names an entry by index; the entry
//! records which processes hold it, and is let go when none does.
//!
//! A stream to 127.0.0.0/8 is a pair of entries, each holding what the other
//! wrote. A datagram to a bound loopback port is queued on that entry. A
//! stream to anywhere else is an entry backed by a net.sockets handle, or by
//! a net.anon stream when the Anyone network is the one chosen.

mod adopt;
mod anon_rx;
mod backend;
mod bind;
mod cell;
mod deliver;
mod free;
mod gram;
mod gram_dest;
mod gram_in;
mod gram_room;
mod gram_target;
mod holders;
mod kinds;
mod link;
mod name;
mod new;
mod opts;
mod opts_more;
mod pair;
mod port;
mod progress;
mod put;
mod ready;
mod recv;
mod send;
mod shut_anon;
mod syn;
mod table;
mod take;
mod types;
mod unlisten;

pub use anon_rx::Got;
pub use backend::{Backend, Close, Via};
pub use cell::with;
pub use gram_dest::Dest;
pub use holders::Holder;
pub use kinds::Link;
pub use name::{Peer, UName};
pub use opts::Opts;
pub use opts_more::More;
pub use progress::{progress, set_progress};
pub use ready::bits;
pub use types::{Addr, Domain, Proto, Sock};
