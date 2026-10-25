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

//! K6: the Command register stores exactly the writable bits a driver
//! names, so Bus Master can be cleared (A10 gap 2: it was ORed in), and Bus
//! Master is refused to a caller without Dma.

use super::command_rule::{at_claim, command_value, Refused, BUS_MASTER, MEMORY_SPACE};

const IO: u16 = 1;

#[test]
fn a_write_of_writable_bits_is_stored_exactly() {
    let on = MEMORY_SPACE | BUS_MASTER;
    assert_eq!(command_value(MEMORY_SPACE, on, true, true), Ok(MEMORY_SPACE), "Bus Master cleared");
    assert_eq!(command_value(0, on, true, true), Ok(0), "both cleared");
    assert_eq!(command_value(MEMORY_SPACE, on | IO, true, true), Ok(MEMORY_SPACE | IO), "I/O kept");
    assert_eq!(command_value(on, MEMORY_SPACE, true, true), Ok(on), "Bus Master set, with Dma");
}

#[test]
fn a_write_carrying_other_bits_leaves_them_as_they_are() {
    let current = IO | MEMORY_SPACE;
    assert_eq!(command_value(IO | MEMORY_SPACE, current, false, true), Ok(current));
    assert_eq!(command_value(MEMORY_SPACE | 1 << 8, current, true, true), Err(Refused::Bits), "SERR");
    assert_eq!(command_value(MEMORY_SPACE, current, true, true), Ok(MEMORY_SPACE | IO));
    assert_eq!(command_value(BUS_MASTER, IO, true, true), Ok(BUS_MASTER | IO), "I/O off is not asked");
}

#[test]
fn bus_master_is_refused_without_dma() {
    assert_eq!(command_value(BUS_MASTER, 0, false, true), Err(Refused::BusMaster));
    assert_eq!(command_value(MEMORY_SPACE | BUS_MASTER, 0, false, true), Err(Refused::BusMaster));
    let on = MEMORY_SPACE | BUS_MASTER;
    assert_eq!(command_value(on, on, false, true), Err(Refused::BusMaster), "nor kept on");
    assert_eq!(command_value(MEMORY_SPACE, on, false, true), Ok(MEMORY_SPACE), "turned off, always");
}

#[test]
fn memory_space_is_refused_without_mmio() {
    assert_eq!(command_value(MEMORY_SPACE, 0, false, false), Err(Refused::MemorySpace));
    assert_eq!(command_value(IO | MEMORY_SPACE, IO, true, false), Err(Refused::MemorySpace));
    let on = IO | MEMORY_SPACE;
    assert_eq!(command_value(0, on, false, false), Ok(IO), "turned off, always");
    assert_eq!(command_value(MEMORY_SPACE, on, false, false), Err(Refused::MemorySpace), "kept");
    assert_eq!(command_value(MEMORY_SPACE, 0, false, true), Ok(MEMORY_SPACE), "with Mmio");
}

#[test]
fn a_port_only_claim_leaves_memory_space_and_bus_master_off() {
    let firmware = IO | MEMORY_SPACE | BUS_MASTER | 1 << 8;
    assert_eq!(at_claim(firmware, true), IO | 1 << 8, "both off, the rest kept");
    assert_eq!(at_claim(firmware, false), IO | MEMORY_SPACE | 1 << 8, "Bus Master alone");
    assert_eq!(at_claim(0, true), 0);
}
