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

//! Whether a remapping unit in service translates a given PCI device.
//!
//! Bring-up programs every segment 0 unit DMAR lists, all or nothing. A device
//! that none of them covers (no INCLUDE_PCI_ALL unit, and no scope naming it) is
//! not translated at all, and would read an IOVA as a physical address: until
//! every unit was programmed that was the RTL8821CE behind the second unit of an
//! Intel laptop, staging its firmware from whatever lived at 0x100000. The
//! broker asks here before it moves a device into a capsule's domain, and leaves
//! an uncovered device on its physical address, counted as unconfined.

use crate::arch::x86_64::acpi::parser::other::{remap_unit_scopes, translated_by_some};

/// PCI segment the config-space transport reaches. Only segment 0 is driven.
const SEGMENT: u16 = 0;

/// Whether a programmed unit translates `bus:dev.func`.
pub fn unit_covers(bus: u8, device: u8, function: u8) -> bool {
    let units = remap_unit_scopes();
    translated_by_some(&units, SEGMENT, (bus, device, function), &bridge_buses)
}

// A type-1 (bridge) function's secondary and subordinate bus numbers, or `None`
// for anything that is not an answering bridge.
fn bridge_buses(bus: u8, device: u8, function: u8) -> Option<(u8, u8)> {
    use crate::drivers::pci::config::access::read8;
    let header = read8(bus, device, function, 0x0E).ok()?;
    if header & 0x7F != 0x01 {
        return None;
    }
    let secondary = read8(bus, device, function, 0x19).ok()?;
    let subordinate = read8(bus, device, function, 0x1A).ok()?;
    if secondary == 0 || secondary == 0xFF || subordinate < secondary {
        return None;
    }
    Some((secondary, subordinate))
}
