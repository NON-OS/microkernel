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

/*
 * Whether an address off the wire can be one end of a TCP connection. A
 * connection is between two unicast hosts; 0/8 (this network), 127/8
 * (loopback, never seen on a wire) and everything from 224 up (multicast,
 * the reserved block and the limited broadcast) are not, and our own
 * address as a source is a forgery (RFC 1122 3.2.1.3, 4.2.3.10).
 */
pub fn usable_pair(src: [u8; 4], dst: [u8; 4], local: [u8; 4]) -> bool {
    unicast(src) && unicast(dst) && src != local
}

fn unicast(a: [u8; 4]) -> bool {
    a[0] != 0 && a[0] != 127 && a[0] < 224
}
