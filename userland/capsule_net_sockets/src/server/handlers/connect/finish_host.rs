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

use crate::clients::nym;
use crate::protocol::{E_NO_HANDLE, E_NO_TRANSPORT, E_OK, OP_CONNECT_HOST};
use crate::server::parse_req::Request;
use crate::sockets::{Kind, RemoteAddr4, SocketKey, SOCKETS};
use crate::state;

use super::{connect_nym, install_transport, pending, status_host};

/// Connect the caller's socket to an address its name resolved to. A stream
/// is answered when its handshake resolves; the others are answered here.
pub fn finish(pid: u32, req: &Request, key: SocketKey, ip: [u8; 4], port: u16, tx: &mut [u8]) {
    let Some(sock) = SOCKETS.with(key, |s| *s) else {
        return status_host::status(pid, req, E_NO_HANDLE, tx);
    };
    let errno = match sock.kind {
        Kind::Datagram => SOCKETS
            .with(key, |s| s.remote = Some(RemoteAddr4 { ip, port }))
            .map_or(E_NO_HANDLE, |_| E_OK),
        Kind::Stream => return pending::start(pid, OP_CONNECT_HOST, req, key, ip, port, tx),
        Kind::Mixnet => match connect_nym::connect_nym() {
            Ok(h) => {
                let e = install_transport::install_transport(key, Kind::Mixnet, ip, port, h);
                if e != E_OK {
                    let _ = nym::close(state::nym(), h);
                }
                e
            }
            Err(_) => E_NO_TRANSPORT,
        },
    };
    status_host::status(pid, req, errno, tx);
}
