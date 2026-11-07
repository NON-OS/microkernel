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

//! Resetting a connection a listener took, once the route is not Direct.
//! smoltcp's abort sends the reset and closes the socket but keeps what the
//! peer had sent and nobody read yet, which a later RECV would still hand
//! out. A cut connection gives up that queue too, so nothing its peer sent
//! in the clear reaches the session after the route changed. Pure over the
//! socket, so net_core_proofs holds it by path.

use smoltcp::socket::tcp;

/// Abort `sock` and discard every byte queued to be read from it.
pub fn reset(sock: &mut tcp::Socket) {
    sock.abort();
    while matches!(sock.recv(|queued| (queued.len(), queued.len())), Ok(n) if n > 0) {}
}
