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

//! With two controllers the multiplexer reads the slot id and root port of
//! every request first. One that names no slot it handed out, no port of
//! either controller, a TT hub not bound on the port's controller, or a
//! place below no hub there is refused before any controller sees a
//! command: an Enable Slot for it used to run first.

use super::bench::Bench;
use crate::protocol::{
    E_INVAL, OP_ADDRESS_DEVICE, OP_ADDRESS_ROUTED, OP_DISABLE_SLOT, OP_ENABLE_SLOT,
    OP_GET_DEVICE_DESCRIPTOR, OP_INTERRUPT_IN,
};

const HIGH_SPEED: u32 = 3;

#[test]
fn requests_naming_nothing_handed_out_are_refused_before_any_command() {
    let mut b = Bench::new(HIGH_SPEED);
    let _second = b.second_controller();
    let v = b.enable_slot();
    let ran = b.ran().len();
    let hostile: [(u16, &[u8]); 11] = [
        (OP_ADDRESS_DEVICE, &[v, 0]),
        (OP_ADDRESS_DEVICE, &[v, 5]),
        (OP_ADDRESS_DEVICE, &[v, 255]),
        (OP_ADDRESS_DEVICE, &[v.wrapping_add(1), 1]),
        (OP_ADDRESS_DEVICE, &[v]),
        (OP_ADDRESS_ROUTED, &[v, 1, 2, 1, 0, 0, 0, 0x7F, 1]),
        (OP_ADDRESS_ROUTED, &[v, 1, 2, 3, 0, 0, 0, 0, 0]),
        (OP_DISABLE_SLOT, &[v.wrapping_add(1)]),
        (OP_GET_DEVICE_DESCRIPTOR, &[v]),
        (OP_INTERRUPT_IN, &[]),
        (OP_ENABLE_SLOT, &[1]),
    ];
    for (op, body) in hostile {
        assert_eq!(b.call(op, body).status, E_INVAL, "{op:#x} {body:x?}");
    }
    assert_eq!(b.ran().len(), ran, "no controller ran a command");
    assert_eq!(b.call(OP_ADDRESS_DEVICE, &[v, 1]).status, 0, "port 1 is the first's");
}
