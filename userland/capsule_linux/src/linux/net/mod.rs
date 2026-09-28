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
mod bind;
mod call_kind;
mod call;
mod close;
mod connect;
mod connect_lo;
mod connect_dgram;
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
mod msg;
mod msg_hdr;
mod name;
mod ops;
mod opt_get;
mod opt_ids;
mod opt_set;
mod opt_time;
mod opt_value;
mod pair;
mod peer_addr;
mod policy;
mod poll;
mod poll_set;
mod poll_socket;
pub mod raw;
pub mod raw_io;
mod resolver;
pub mod route;
mod select;
mod shutdown;
pub mod sock;
mod sockaddr;
mod sockaddr_out;
mod sockaddr_un;
mod socket;
mod stream;
mod try_call;
mod unix_bind;
mod unix_calls;
mod unix_name;
mod xfer_in;
mod xfer_out;

pub use accept::accept4;
pub use bind::bind;
pub use call_kind::{flags as call_flags, wants_all};
pub use listen::listen;
pub use close::close;
pub use connect::connect;
pub use dgram::{recvfrom, sendto};
pub use fd::{is_stream, sock_id};
pub use mmsg::{recvmmsg, sendmmsg};
pub use msg::{recvmsg, sendmsg};
pub use name::{getpeername, getsockname};
pub use opt_get::getsockopt;
pub use opt_set::setsockopt;
pub use opt_time::limit_ms;
pub use pair::socketpair;
pub use poll::{ready, POLLERR, POLLHUP};
pub use poll_set::poll;
pub use poll_socket::outside;
pub use select::{clear as select_clear, select};
pub use shutdown::shutdown;
pub use socket::socket;
pub use try_call::try_call;
pub use xfer_in::read as recv;
pub use xfer_out::write as send;
