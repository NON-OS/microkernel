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

//! How long a TCP connection may go unanswered before net.core gives it up.
//!
//! smoltcp sets neither by default, so a connection whose peer or NAT forgot
//! it was never noticed: an idle one sat open forever, and one with data in
//! flight retransmitted forever. A router reboot drops every NAT mapping, so
//! a long-lived connection (the onion transports' circuits, a kept-alive page
//! fetch) stayed half open and its owner waited on it instead of dialling
//! again.

use smoltcp::socket::tcp::Socket;
use smoltcp::time::Duration;

/// An idle connection is probed this often. RFC 1122 4.2.3.6 allows a
/// keep-alive; 75 s is Linux's tcp_keepalive_intvl. Each answered probe
/// restarts TIMEOUT, so a healthy idle connection is never given up.
pub const KEEP_ALIVE: Duration = Duration::from_secs(75);

/// No segment from the peer for this long, idle or with data in flight, and
/// the connection is aborted. Five minutes outlasts a router reboot (one to
/// two minutes on home routers), so a connection that can resume does.
pub const TIMEOUT: Duration = Duration::from_secs(300);

pub fn arm(sock: &mut Socket) {
    sock.set_keep_alive(Some(KEEP_ALIVE));
    sock.set_timeout(Some(TIMEOUT));
}
