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

//! Telling the guard a circuit is finished with.

use crate::cell::{Cell, CELL_DESTROY};

/*
 * tor-spec 5.4: a circuit is torn down by sending DESTROY. Without it the relays
 * hold the circuit open until their own timeouts expire, so a client that retires
 * three circuits an hour leaves three times that many sitting on other people's
 * machines, and every other client on the network sends one.
 *
 * The reason is NONE. The payload allows a reason octet and this side's reasons
 * are its own scheduling: a circuit retired on age looks different from one
 * retired after a failed exit, and saying which is a distinguisher offered for no
 * benefit. NONE is what a client that will not say looks like.
 */

const REASON_NONE: u8 = 0;

/// A DESTROY cell for `circuit`, ready to send on the link that carries it.
pub fn destroy(circuit: u32) -> Cell {
    let mut cell = Cell::new(circuit, CELL_DESTROY);
    cell.payload[0] = REASON_NONE;
    cell
}
