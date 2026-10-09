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

//! The DESTROY cell a client sends to tear a circuit down.

use crate::cell::CELL_DESTROY;
use crate::circuit::destroy;

/// Command 4 on the circuit being torn down, per tor-spec 5.4.
#[test]
fn a_destroy_names_its_circuit_and_command() {
    let cell = destroy(0x8000_002A);
    assert_eq!(cell.command, CELL_DESTROY);
    assert_eq!(cell.circuit, 0x8000_002A, "the circuit being torn down, not another");
}

#[test]
fn a_destroy_says_nothing_about_why() {
    let cell = destroy(1);
    assert_eq!(cell.payload[0], 0, "reason NONE");
    assert!(cell.payload[1..].iter().all(|b| *b == 0), "and nothing else in the payload");
}

#[test]
fn the_client_high_bit_survives() {
    assert_eq!(destroy(0x8000_0001).circuit & 0x8000_0000, 0x8000_0000);
}
