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

//! Reading a RELAY_CONNECTED body.

/*
 * tor-spec section 6.2: for IPv4 the body is a four byte address and a four byte
 * TTL. For IPv6 the first four bytes are zero, then an address type of 6, then
 * sixteen address bytes and the TTL.
 *
 * Nothing here needs the address. A client that asked for a name does not learn
 * anything it can use from the exit's resolution, and keeping it would only give
 * the caller a value it might be tempted to trust. The body is parsed solely to
 * refuse one that is malformed.
 */
/// Whether a CONNECTED body is well formed.
pub fn is_valid(body: &[u8]) -> bool {
    match body.len() {
        0 => true,
        8 => true,
        n if n >= 25 => body[..4] == [0, 0, 0, 0] && body[4] == 6,
        _ => false,
    }
}
