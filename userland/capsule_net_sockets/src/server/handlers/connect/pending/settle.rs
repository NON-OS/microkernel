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

//! Answering a connect whose handshake resolved.

use crate::clients::tcp;
use crate::protocol::{E_NO_TRANSPORT, E_OK};
use crate::server::respond::respond_to;
use crate::sockets::Kind;
use crate::state;

use super::super::install_transport::{install_transport, uninstall_transport};
use super::table::Pending;
use super::verdict::kept;

/// The handshake completed: attach the connection to the caller's socket.
/// A socket that can no longer take it, or a caller that gave up on the
/// connect before the answer came (it reached no one), gets the connection
/// closed, not leaked, and the socket is left as the caller believes it is.
pub(super) fn established(p: Pending, tx: &mut [u8]) {
    let errno = install_transport(p.key, Kind::Stream, p.ip, p.port, p.transport);
    let heard = respond_to(p.ipc, p.op, errno, p.request_id, tx) >= 0;
    if kept(errno == E_OK, heard) {
        return;
    }
    if errno == E_OK {
        uninstall_transport(p.key);
    }
    let _ = tcp::close(state::tcp(), p.transport);
}

/// The peer refused, the connection died, or the deadline passed.
pub(super) fn failed(p: Pending, tx: &mut [u8]) {
    let _ = tcp::close(state::tcp(), p.transport);
    let _ = respond_to(p.ipc, p.op, E_NO_TRANSPORT, p.request_id, tx);
}
