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

//! The NETINFO cell that closes the link handshake.

use crate::cell::{Cell, CELL_NETINFO};

/// Address type for an IPv4 address, tor-spec section 6.4.
const ADDRESS_IPV4: u8 = 4;

/// Our NETINFO, sent on circuit zero to finish the handshake.
///
pub fn ours(their_address: [u8; 4]) -> Cell {
    let mut cell = Cell::new(0, CELL_NETINFO);
    // timestamp, four bytes of zero, then the address we reached them at.
    cell.payload[4] = ADDRESS_IPV4;
    cell.payload[5] = 4;
    cell.payload[6..10].copy_from_slice(&their_address);
    // Our own address count, zero.
    cell.payload[10] = 0;
    cell
}
