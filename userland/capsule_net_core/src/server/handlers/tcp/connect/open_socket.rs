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

use smoltcp::socket::tcp;

use crate::handles;
use crate::server::handlers::tcp::connect::ephemeral;
use crate::server::handlers::tcp::connect::types::{ConnectOutcome, Endpoint};
use crate::state;

/*
 * The receive buffer is the window the peer may fill before it has to wait
 * for an ACK. At 8 KiB the window sat near zero for a whole page load and
 * the sender idled between the reader's drains; 64 KiB lets a page arrive
 * while the reader is busy, and smoltcp scales the window past 65,535 bytes.
 * Requests are small, so sending keeps a smaller buffer.
 */
const RX_BYTES: usize = 64 * 1024;
const TX_BYTES: usize = 16 * 1024;

pub fn open_socket(sender_pid: u32, endpoint: Endpoint) -> ConnectOutcome {
    match state::with_iface(|iface, sockets, _dev| {
        let rx = tcp::SocketBuffer::new(alloc::vec![0u8; RX_BYTES]);
        let tx_buf = tcp::SocketBuffer::new(alloc::vec![0u8; TX_BYTES]);
        let mut sock = tcp::Socket::new(rx, tx_buf);
        let local = ephemeral::next_ephemeral();
        if sock.connect(iface.context(), (endpoint.remote, endpoint.port), local).is_err() {
            return ConnectOutcome::ConnectFailed;
        }
        let handle = sockets.add(sock);
        match handles::alloc(sender_pid, handle) {
            Some(app_handle) => ConnectOutcome::Ok(app_handle),
            None => {
                sockets.remove(handle);
                ConnectOutcome::TableFull
            }
        }
    }) {
        Some(outcome) => outcome,
        None => ConnectOutcome::TableFull,
    }
}
