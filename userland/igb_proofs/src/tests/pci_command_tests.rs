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

//! The PCI Command value written after the claim: Memory Space and Bus
//! Master on, and every bit the record already needed kept.

use crate::setup::pci_command::command_value;

const IO: u16 = 1 << 0;
const MEMORY: u16 = 1 << 1;
const BUS_MASTER: u16 = 1 << 2;

#[test]
fn memory_space_and_bus_master_are_always_set() {
    assert_eq!(command_value(0), MEMORY | BUS_MASTER);
    assert_eq!(command_value(MEMORY), MEMORY | BUS_MASTER);
}

#[test]
fn an_io_decode_the_record_needs_is_kept() {
    assert_eq!(command_value(IO | MEMORY), IO | MEMORY | BUS_MASTER);
}

#[test]
fn no_bit_the_record_held_is_ever_cleared() {
    for bits in 0..=u16::MAX {
        let v = command_value(bits);
        assert_eq!(v & bits, bits);
        assert_eq!(v & !bits & !(MEMORY | BUS_MASTER), 0, "nothing else is added");
    }
}
