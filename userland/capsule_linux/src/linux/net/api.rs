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

//! What the rest of the personality calls on sockets.

pub use super::accept::accept4;
pub use super::bind::bind;
pub use super::call_kind::{flags as call_flags, wants_all};
pub use super::close::close;
pub use super::connect::connect;
pub use super::dgram::sendto;
pub use super::fd::{is_stream, sock_id};
pub use super::listen::listen;
pub use super::mmsg::{recvmmsg, sendmmsg};
pub use super::msg::sendmsg;
pub use super::msg_recv::recvmsg;
pub use super::name::{getpeername, getsockname};
pub use super::opt::{getsockopt, limit_ms, setsockopt};
pub use super::pair::socketpair;
pub use super::poll::{ready, POLLERR, POLLHUP};
pub use super::poll_set::poll;
pub use super::poll_socket::outside;
pub use super::recvfrom::recvfrom;
pub use super::select::{clear as select_clear, select};
pub use super::shutdown::shutdown;
pub use super::socket::socket;
pub use super::try_call::try_call;
pub use super::xfer_in::read as recv;
pub use super::xfer_out::write as send;
