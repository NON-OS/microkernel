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
//! and nothing outside the broker's allowlist, so it is never refused and,
//! once the broker stores it as written, "off" turns Bus Master off.

use crate::setup::pci_command::command_value;

const MEMORY: u16 = 1 << 1;
const BUS_MASTER: u16 = 1 << 2;
const WRITABLE: u16 = MEMORY | BUS_MASTER | (1 << 10);

#[test]
fn memory_space_always_and_bus_master_only_when_asked() {
    assert_eq!(command_value(true), MEMORY | BUS_MASTER);
    assert_eq!(command_value(false), MEMORY);
}

#[test]
fn every_firmware_state_takes_the_value_and_keeps_its_other_bits() {
    for current in 0..=u16::MAX {
        for on in [false, true] {
            let v = command_value(on);
            assert_eq!(v & !WRITABLE, 0, "only allowlisted bits are written");
            // Today's broker ORs the value in; the fixed one stores it.
            assert_eq!((current | v) & !WRITABLE, current & !WRITABLE);
            let stored = (current & !WRITABLE) | v;
            assert_eq!(stored & BUS_MASTER != 0, on);
        }
    }
}
