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

use crate::protocol::OP_CONNECT;
use crate::server::parse_req::Request;
use crate::sockets::SocketKey;

use super::pending;

/// A blocking connect by address. The caller's reply waits until the
/// handshake resolves; this service does not.
pub fn update_stream(
    pid: u32,
    req: &Request,
    key: SocketKey,
    ip: [u8; 4],
    port: u16,
    tx: &mut [u8],
) {
    pending::start(pid, OP_CONNECT, req, key, ip, port, tx);
}
