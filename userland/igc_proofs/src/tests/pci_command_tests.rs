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

//! The PCI Command value: Memory Space always, Bus Master only when asked,
//! and nothing outside the broker's allowlist, so it is never refused. It
//! goes in by the masked form, which names both bits, so "off" turns Bus
//! Master off; the plain form ORs the value in and could not.

use crate::setup::pci_command::{command_value, COMMAND_MASK};

const MEMORY: u16 = 1 << 1;
const BUS_MASTER: u16 = 1 << 2;
const WRITABLE: u16 = MEMORY | BUS_MASTER | (1 << 10);

#[test]
fn memory_space_always_and_bus_master_only_when_asked() {
    assert_eq!(command_value(true), MEMORY | BUS_MASTER);
    assert_eq!(command_value(false), MEMORY);
    assert_eq!(COMMAND_MASK, MEMORY | BUS_MASTER);
}

#[test]
fn every_firmware_state_takes_the_value_and_keeps_its_other_bits() {
    assert_eq!(COMMAND_MASK & !WRITABLE, 0, "the mask names only allowlisted bits");
    for current in 0..=u16::MAX {
        for on in [false, true] {
            let v = command_value(on);
            assert_eq!(v & !COMMAND_MASK, 0, "the value sets no bit the mask leaves out");
            // The masked form, as the broker merges it.
            let stored = (current & !COMMAND_MASK) | v;
            assert_eq!(stored & !COMMAND_MASK, current & !COMMAND_MASK);
            assert_eq!(stored & BUS_MASTER != 0, on);
            // The plain form would OR it in, and never clear Bus Master.
            assert_eq!((current | v) & BUS_MASTER != 0, on || current & BUS_MASTER != 0);
        }
    }
}
