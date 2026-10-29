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

//! Where a box may bind and listen, decided from the policy alone. With
//! NET_LISTEN_LO anywhere on loopback (net/bind.rs holds it to 127.0.0.0/8).
//! With SERVE_ONE and without it, one stream listener only, on exactly
//! 127.0.0.1 at the port its field names, which the kernel serves to the one
//! capsule the field names (net/serve_one). SERVE_ONE is no NET_LISTEN_LO:
//! every other bind stays refused.

use nonos_linux_policy::mask::{grants, NET_LISTEN_LO, SERVE_ONE};
use nonos_linux_policy::ServeOne;

/// The one address a SERVE_ONE listener is on.
pub const SERVE_IP: [u8; 4] = [127, 0, 0, 1];

/// Whether a bind to `ip`:`port` is taken, `stream` for a stream socket.
pub fn may_bind(mask: u64, field: Option<ServeOne>, stream: bool, ip: [u8; 4], port: u16) -> bool {
    grants(mask, NET_LISTEN_LO) || serve_at(mask, field, stream, ip, port)
}

/// Whether `ip`:`port`, for a stream, is the listener SERVE_ONE's field names.
pub fn serve_at(mask: u64, field: Option<ServeOne>, stream: bool, ip: [u8; 4], port: u16) -> bool {
    let named = field.is_some_and(|f| f.port == port);
    grants(mask, SERVE_ONE) && stream && ip == SERVE_IP && named
}
