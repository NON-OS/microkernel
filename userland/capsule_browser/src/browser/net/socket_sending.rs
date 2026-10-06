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

//! Whether what was written on a socket is still on its way.

/// True while bytes written on `handle` wait for the proxy to answer them,
/// after asking it once more. A socket through net.sockets takes a send in
/// the call that makes it, so it is never still sending. An error when the
/// proxy refused the call outright and the bytes can never be answered.
pub fn socket_sending(handle: u32) -> Result<bool, ()> {
    if super::mixnet::is_proxied(handle) {
        return super::mixnet::sending(handle);
    }
    Ok(false)
}
