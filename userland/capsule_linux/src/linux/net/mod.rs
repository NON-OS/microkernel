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
//! family, which net.sockets carries over the mixnet.

mod accept;
mod api;
mod bind;
mod call;
mod call_kind;
mod cap;
mod close;
mod connect;
mod connect_dgram;
mod connect_dial;
mod connect_lo;
mod connect_out;
mod dest;
mod dgram;
mod dgram_addr;
pub mod dns;
mod fd;
mod flags;
mod host_body;
mod iov;
mod listen;
mod mmsg;
mod mmsg_each;
mod msg;
mod msg_hdr;
mod msg_recv;
mod name;
mod named;
mod offline;
mod ops;
mod opt;
mod pair;
mod peer_addr;
mod policy;
mod poll;
mod poll_set;
mod poll_socket;
pub mod raw;
pub mod raw_io;
mod recvfrom;
mod resolver;
pub mod route;
mod select;
mod shutdown;
pub mod sock;
mod sockaddr;
mod sockaddr_out;
mod socket;
mod stream;
mod try_call;
mod xfer_in;
mod xfer_out;

pub use api::*;
pub use offline::{any_inet, refuse_socket};
